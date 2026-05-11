use std::fmt;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::{
    AssignmentQueue, ColumnIndex, ConflictGraph, CrossUnitCollisionReport, CustomKeys,
    GridCoordinate, GridLayout, NamedCommandGrid, RowIndex, UnitCollisionReport, UnitGrids,
    UnitKeyedCustomKeys,
};

#[derive(Parser)]
#[command(name = "warcraft", about = "Query the Warcraft III game database")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Show command grids and collision report for a unit.
    Unit {
        /// Warcraft III unit or hero ID (e.g. Hpal, hpea, etol, hfoo).
        unit_id: String,
        /// Path to a CustomKeys.txt file. Defaults to the built-in game defaults.
        #[arg(long, short, value_name = "FILE")]
        keys: Option<PathBuf>,
    },
    /// Parse and display the contents of a CustomKeys.txt file.
    Keys {
        /// Path to a CustomKeys.txt file.
        file: PathBuf,
        /// Show only entries for a specific unit ID (e.g. Hpal, hpea).
        #[arg(long, short, value_name = "UNIT")]
        unit: Option<String>,
        /// Apply the position cascade using this keyboard layout.
        /// Use "qwerty" for the standard QWERTY grid, or a 12-character
        /// row-major string (e.g. "QWERASDFZXCV").
        #[arg(long, short, value_name = "LAYOUT")]
        layout: Option<String>,
        /// Show all position and hotkey collisions instead of the binding list.
        #[arg(long, short)]
        collisions: bool,
        /// Show cross-unit collisions: positions where multiple abilities overlap
        /// globally, with every unit affected and every unit that carries each ability.
        #[arg(long, short = 'x')]
        cross: bool,
        /// Show the conflict graph: node/edge counts, top carriers, top degree nodes.
        #[arg(long, short = 'g')]
        graph: bool,
        /// Show the assignment queue: ordered list of positions to resolve and which
        /// abilities are anchors vs movers.
        #[arg(long, short = 'q')]
        queue: bool,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Unit { unit_id, keys } => {
            let custom_keys = match keys {
                None => CustomKeys::from("").normalize(),
                Some(path) => CustomKeys::try_from(path.as_path())
                    .unwrap_or_else(|error| {
                        eprintln!("error: cannot read {}: {error}", path.display());
                        std::process::exit(1);
                    })
                    .normalize(),
            };
            inspect_unit(unit_id, custom_keys);
        }
        Command::Keys {
            file,
            unit,
            layout,
            collisions,
            cross,
            graph,
            queue,
        } => {
            let mut custom_keys = CustomKeys::try_from(file.as_path())
                .unwrap_or_else(|error| {
                    eprintln!("error: cannot read {}: {error}", file.display());
                    std::process::exit(1);
                })
                .normalize();
            let grid_layout = layout
                .as_deref()
                .map(grid_layout_from_spec)
                .unwrap_or_else(GridLayout::qwerty_grid);
            if layout.is_some() {
                custom_keys.apply_grid_to_all_bindings(grid_layout);
            }
            if graph {
                let conflict_graph = ConflictGraph::build(&custom_keys);
                println!("{conflict_graph}");
            } else if queue {
                let conflict_graph = ConflictGraph::build(&custom_keys);
                let assignment_queue = AssignmentQueue::build(conflict_graph);
                println!("{assignment_queue}");
            } else if cross {
                let report = CrossUnitCollisionReport::compute(&custom_keys);
                println!("{report}");
            } else if collisions {
                let report = UnitCollisionReport::compute(&custom_keys, grid_layout);
                let display = match unit.as_deref() {
                    None => report,
                    Some(unit_id) => report.for_unit(unit_id),
                };
                println!("{display}");
            } else {
                let unit_keyed = UnitKeyedCustomKeys::from(&custom_keys);
                let display = match unit.as_deref() {
                    None => unit_keyed,
                    Some(unit_id) => unit_keyed.for_unit(unit_id),
                };
                println!("{display}");
            }
        }
    }
}

fn grid_layout_from_spec(spec: &str) -> GridLayout {
    match spec {
        "qwerty" => GridLayout::qwerty_grid(),
        other => GridLayout::try_from(other).unwrap_or_else(|_| {
            eprintln!(
                "error: invalid layout {other:?} — use \"qwerty\" or a 12-character \
                 row-major string (e.g. \"QWERASDFZXCV\")"
            );
            std::process::exit(1);
        }),
    }
}

fn inspect_unit(unit_id_string: String, custom_keys: CustomKeys) {
    let leaked: &'static mut str = Box::leak(unit_id_string.clone().into_boxed_str());
    let static_id: &'static str = leaked;
    let unit_id = WarcraftObjectId::from(static_id);

    let unit_grids = UnitGrids::for_unit(unit_id);
    let layout = GridLayout::qwerty_grid();

    println!("Unit: {unit_id_string}");

    for (index, named_grid) in unit_grids.grids().iter().enumerate() {
        let display = CommandGridDisplay::new(named_grid, &custom_keys, layout);
        println!("\n[{index}] {:?}", named_grid.role());
        println!("{display}");
    }

    let position_cards = unit_grids.position_collisions(&custom_keys);
    let hotkey_cards = unit_grids.hotkey_collisions(&custom_keys, layout);

    let no_position_collisions = position_cards.iter().all(|card| card.is_empty());
    let no_hotkey_collisions = hotkey_cards.iter().all(|card| card.is_empty());
    if no_position_collisions && no_hotkey_collisions {
        println!("\nNo collisions.");
        return;
    }

    if !no_position_collisions {
        println!("\nPosition collisions:");
        for card in position_cards {
            for (position, collision_slots) in card {
                let slot_list: Vec<&str> = collision_slots.iter().map(|s| s.as_str()).collect();
                let column = u8::from(position.column());
                let row = u8::from(position.row());
                println!(
                    "  {:?}  ({column},{row})  {}",
                    card.role(),
                    slot_list.join(", ")
                );
            }
        }
    }

    if !no_hotkey_collisions {
        println!("\nHotkey collisions:");
        for card in hotkey_cards {
            for (_, entry) in card {
                let slot_list: Vec<&str> = entry.slots().iter().map(|s| s.as_str()).collect();
                println!(
                    "  {:?}  key {}  {}",
                    card.role(),
                    entry.token(),
                    slot_list.join(", ")
                );
            }
        }
    }
}

struct CommandGridDisplay<'a> {
    grid: &'a NamedCommandGrid,
    custom_keys: &'a CustomKeys,
    layout: GridLayout,
}

impl<'a> CommandGridDisplay<'a> {
    fn new(grid: &'a NamedCommandGrid, custom_keys: &'a CustomKeys, layout: GridLayout) -> Self {
        Self {
            grid,
            custom_keys,
            layout,
        }
    }
}

impl fmt::Display for CommandGridDisplay<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        const COLUMNS: [ColumnIndex; 4] = [
            ColumnIndex::Zero,
            ColumnIndex::One,
            ColumnIndex::Two,
            ColumnIndex::Three,
        ];
        const ROWS: [RowIndex; 3] = [RowIndex::Zero, RowIndex::One, RowIndex::Two];
        const CELL: usize = 18;
        const ID_WIDTH: usize = 12;
        const KEY_WIDTH: usize = 3;

        let is_research = self.grid.role().is_research_context();
        let card = self.grid.card();

        let bar = "─".repeat(CELL);
        let bars: Vec<&str> = vec![bar.as_str(); 4];
        let top = format!("┌{}┐", bars.join("┬"));
        let middle = format!("├{}┤", bars.join("┼"));
        let bottom = format!("└{}┘", bars.join("┴"));

        writeln!(formatter, "{top}")?;
        for (row_index, row) in ROWS.iter().enumerate() {
            let cells: Vec<String> = COLUMNS
                .iter()
                .map(|col| {
                    let position = GridCoordinate::new(*col, *row);
                    match card.slot_at(position) {
                        None => format!("{:CELL$}", ""),
                        Some(slot) => {
                            let token = self.custom_keys.effective_hotkey_token(
                                &slot,
                                self.layout,
                                is_research,
                            );
                            let key = token
                                .map(|t| t.to_string())
                                .unwrap_or_else(|| " ".to_string());
                            let id = slot.as_str();
                            let id_truncated = if id.len() > ID_WIDTH {
                                &id[..ID_WIDTH]
                            } else {
                                id
                            };
                            format!(" {id_truncated:<ID_WIDTH$} {key:<KEY_WIDTH$} ")
                        }
                    }
                })
                .collect();
            writeln!(formatter, "│{}│", cells.join("│"))?;
            if row_index < ROWS.len() - 1 {
                writeln!(formatter, "{middle}")?;
            }
        }
        write!(formatter, "{bottom}")
    }
}
