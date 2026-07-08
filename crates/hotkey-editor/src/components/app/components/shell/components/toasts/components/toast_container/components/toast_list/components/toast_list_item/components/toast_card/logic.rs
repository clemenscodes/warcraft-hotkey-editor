use super::components::error_toast_card::ErrorToastCardProps;
use super::components::info_toast_card::InfoToastCardProps;
use super::components::shared::toast_close::ToastCloseProps;
use super::components::shared::toast_description::ToastDescriptionProps;
use super::components::success_toast_card::SuccessToastCardProps;
use super::components::warning_toast_card::WarningToastCardProps;
use super::props::ToastCardProps;

impl From<&ToastCardProps> for SuccessToastCardProps {
    fn from(props: &ToastCardProps) -> Self {
        let title = props.record.title().to_string();
        let description = ToastDescriptionProps::from(props);
        let close = ToastCloseProps::from(props);
        Self {
            title,
            description,
            close,
        }
    }
}

impl From<&ToastCardProps> for ErrorToastCardProps {
    fn from(props: &ToastCardProps) -> Self {
        let title = props.record.title().to_string();
        let description = ToastDescriptionProps::from(props);
        let close = ToastCloseProps::from(props);
        Self {
            title,
            description,
            close,
        }
    }
}

impl From<&ToastCardProps> for WarningToastCardProps {
    fn from(props: &ToastCardProps) -> Self {
        let title = props.record.title().to_string();
        let description = ToastDescriptionProps::from(props);
        let close = ToastCloseProps::from(props);
        Self {
            title,
            description,
            close,
        }
    }
}

impl From<&ToastCardProps> for InfoToastCardProps {
    fn from(props: &ToastCardProps) -> Self {
        let title = props.record.title().to_string();
        let description = ToastDescriptionProps::from(props);
        let close = ToastCloseProps::from(props);
        Self {
            title,
            description,
            close,
        }
    }
}
