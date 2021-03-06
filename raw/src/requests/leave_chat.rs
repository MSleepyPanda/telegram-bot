use types::*;
use requests::*;

/// Use this method for your bot to leave a group, supergroup or channel.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct LeaveChat<'c> {
    chat_id: ChatRef<'c>
}

impl<'c> Request for LeaveChat<'c> {
    type Response = TrueToUnitResponse;

    fn name(&self) -> &'static str {
        "leaveChat"
    }
}

impl<'c> LeaveChat<'c> {
    pub fn new<C>(chat: C) -> Self where C: ToChatRef<'c> {
        LeaveChat {
            chat_id: chat.to_chat_ref()
        }
    }
}

pub trait CanLeaveChat<'c> {
    fn leave(&self) -> LeaveChat<'c>;
}

impl<'c, C> CanLeaveChat<'c> for C where C: ToChatRef<'c> {
    fn leave(&self) -> LeaveChat<'c> {
        LeaveChat::new(self)
    }
}
