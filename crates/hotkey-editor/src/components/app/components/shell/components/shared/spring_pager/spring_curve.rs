use std::f64::consts::TAU;
use std::fmt;

// Points sampled along a curve. The browser interpolates linearly between them,
// so this many reads as one smooth motion at every duration the pager uses.
const SAMPLE_COUNT: u32 = 48;
// How close to rest, in progress units, the spring must be before it counts as
// settled and the curve ends.
const SETTLED_DISPLACEMENT: f64 = 0.002;
// The step used to search for the moment the spring settles.
const SETTLE_SEARCH_STEP_MS: f64 = 8.0;
// No curve runs longer than this, however slowly its spring settles.
const LONGEST_DURATION_MS: f64 = 1400.0;
// The fastest a curve may start, as a multiple of its spring's angular
// frequency. A flick that only has a few pixels left to travel would otherwise
// throw the card far past its target before the spring pulls it back.
const FASTEST_START_PER_ANGULAR_FREQUENCY: f64 = 3.0;

/// How a spring moves: how strongly it is damped (below 1 it overshoots and
/// swings back, at 1 it comes to rest without overshooting) and how long one
/// undamped swing would take.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
pub(crate) struct SpringCharacter {
    damping_ratio: f64,
    period_ms: f64,
}

impl SpringCharacter {
    pub(crate) const fn new(damping_ratio: f64, period_ms: f64) -> Self {
        Self {
            damping_ratio,
            period_ms,
        }
    }

    fn angular_frequency(&self) -> f64 {
        TAU / self.period_ms
    }
}

/// A spring released toward its target, as the displacement from the target it
/// has at any moment. Progress runs from 0 at the start to 1 at the target, so
/// the displacement starts at -1 and swings to 0.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
struct SpringMotion {
    character: SpringCharacter,
    initial_velocity: f64,
}

impl SpringMotion {
    /// The displacement from the target `elapsed_ms` after the release.
    fn displacement(&self, elapsed_ms: f64) -> f64 {
        let angular_frequency = self.character.angular_frequency();
        let damping_ratio = self.character.damping_ratio;
        let decay = (-damping_ratio * angular_frequency * elapsed_ms).exp();
        if damping_ratio < 1.0 {
            let damped_frequency = angular_frequency * (1.0 - damping_ratio * damping_ratio).sqrt();
            let sine_weight =
                (self.initial_velocity - damping_ratio * angular_frequency) / damped_frequency;
            let phase = damped_frequency * elapsed_ms;
            return decay * (sine_weight * phase.sin() - phase.cos());
        }
        let linear_weight = self.initial_velocity - angular_frequency;
        decay * (linear_weight * elapsed_ms - 1.0)
    }

    /// An upper bound on how far from the target the spring can still be
    /// `elapsed_ms` after the release.
    fn displacement_bound(&self, elapsed_ms: f64) -> f64 {
        let angular_frequency = self.character.angular_frequency();
        let damping_ratio = self.character.damping_ratio;
        let decay = (-damping_ratio * angular_frequency * elapsed_ms).exp();
        if damping_ratio < 1.0 {
            let damped_frequency = angular_frequency * (1.0 - damping_ratio * damping_ratio).sqrt();
            let sine_weight =
                (self.initial_velocity - damping_ratio * angular_frequency) / damped_frequency;
            return decay * (1.0 + sine_weight.abs());
        }
        let linear_weight = self.initial_velocity - angular_frequency;
        decay * (1.0 + linear_weight.abs() * elapsed_ms)
    }

    /// How long the spring takes to settle, never longer than the longest curve.
    fn settle_duration_ms(&self) -> f64 {
        let mut elapsed_ms = SETTLE_SEARCH_STEP_MS;
        while elapsed_ms < LONGEST_DURATION_MS {
            if self.displacement_bound(elapsed_ms) < SETTLED_DISPLACEMENT {
                return elapsed_ms;
            }
            elapsed_ms += SETTLE_SEARCH_STEP_MS;
        }
        LONGEST_DURATION_MS
    }
}

/// A spring's run from where a card is to where it comes to rest, sampled into
/// a CSS `linear()` easing so the browser plays it on the compositor. Its
/// `Display` is the `transition` value that plays it on `transform`.
#[derive(Clone, Debug, PartialEq, PartialOrd, Default)]
pub(crate) struct SpringCurve {
    duration_ms: f64,
    progress_samples: Vec<f64>,
}

impl SpringCurve {
    /// The curve of a spring with `character` that starts with
    /// `initial_velocity`, in progress per millisecond: the speed the finger had,
    /// divided by the distance left to travel, so the card leaves the finger at
    /// exactly the speed the finger had.
    pub(crate) fn new(character: SpringCharacter, initial_velocity: f64) -> Self {
        let fastest_start = character.angular_frequency() * FASTEST_START_PER_ANGULAR_FREQUENCY;
        let bounded_velocity = initial_velocity.clamp(-fastest_start, fastest_start);
        let motion = SpringMotion {
            character,
            initial_velocity: bounded_velocity,
        };
        let duration_ms = motion.settle_duration_ms();
        let sample_span = f64::from(SAMPLE_COUNT);
        let mut progress_samples: Vec<f64> = (0..SAMPLE_COUNT)
            .map(|sample_index| {
                let sample_fraction = f64::from(sample_index) / sample_span;
                let elapsed_ms = duration_ms * sample_fraction;
                1.0 + motion.displacement(elapsed_ms)
            })
            .collect();
        progress_samples.push(1.0);
        Self {
            duration_ms,
            progress_samples,
        }
    }
}

impl SpringCurve {
    pub(crate) fn duration(&self) -> CurveDuration {
        CurveDuration::from(self.duration_ms)
    }
}

/// How long a curve runs, in whole milliseconds, rounded up so a wait of this
/// long never ends before the curve does.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub(crate) struct CurveDuration {
    milliseconds: u32,
}

impl CurveDuration {
    pub(crate) fn milliseconds(&self) -> u32 {
        self.milliseconds
    }
}

impl From<f64> for CurveDuration {
    fn from(duration_ms: f64) -> Self {
        let rounded_up = duration_ms.ceil();
        Self {
            milliseconds: rounded_up as u32,
        }
    }
}

impl fmt::Display for SpringCurve {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "transform {:.0}ms linear(", self.duration_ms)?;
        for (sample_index, progress) in self.progress_samples.iter().enumerate() {
            if sample_index > 0 {
                write!(formatter, ", ")?;
            }
            write!(formatter, "{progress:.4}")?;
        }
        write!(formatter, ")")
    }
}

#[cfg(test)]
mod tests {
    use super::{SpringCharacter, SpringCurve};

    const BOUNCY: SpringCharacter = SpringCharacter::new(0.7, 500.0);
    const SMOOTH: SpringCharacter = SpringCharacter::new(1.0, 500.0);

    #[test]
    fn starts_where_the_card_is_and_ends_on_its_target() {
        let curve = SpringCurve::new(BOUNCY, 0.0);
        let samples = &curve.progress_samples;
        let first_sample = samples.first().copied();
        let last_sample = samples.last().copied();
        assert_eq!(first_sample, Some(0.0));
        assert_eq!(last_sample, Some(1.0));
    }

    #[test]
    fn a_bouncy_spring_overshoots_its_target() {
        let curve = SpringCurve::new(BOUNCY, 0.0);
        let overshoots = curve
            .progress_samples
            .iter()
            .any(|progress| *progress > 1.01);
        assert!(overshoots);
    }

    #[test]
    fn a_smooth_spring_from_rest_never_overshoots() {
        let curve = SpringCurve::new(SMOOTH, 0.0);
        let overshoots = curve
            .progress_samples
            .iter()
            .any(|progress| *progress > 1.0001);
        assert!(!overshoots);
    }

    #[test]
    fn a_thrown_card_moves_faster_at_first() {
        let resting = SpringCurve::new(BOUNCY, 0.0);
        let thrown = SpringCurve::new(BOUNCY, 0.01);
        let resting_second = resting.progress_samples.get(1).copied().unwrap_or_default();
        let thrown_second = thrown.progress_samples.get(1).copied().unwrap_or_default();
        assert!(thrown_second > resting_second);
    }

    #[test]
    fn renders_as_a_transform_transition() {
        let curve = SpringCurve::new(SMOOTH, 0.0);
        let transition = curve.to_string();
        assert!(transition.starts_with("transform "));
        assert!(transition.contains("ms linear(0.0000, "));
        assert!(transition.ends_with(", 1.0000)"));
    }
}
