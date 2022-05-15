use crate::domain::subscriber_email::SubscriberEmail;
use crate::domain::subscriber_name::SubscribeName;

pub struct NewSubscriber {
    pub email: SubscriberEmail,
    pub name: SubscribeName,
}
