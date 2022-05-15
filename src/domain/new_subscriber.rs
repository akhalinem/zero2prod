use crate::domain::subscriber_name::SubscribeName;

pub struct NewSubscriber {
    pub email: String,
    pub name: SubscribeName,
}
