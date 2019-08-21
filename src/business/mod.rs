use serenity::model::channel::Message;

pub(crate) fn buy_responce(_: &Message) -> String {
    String::from("It looks like you want to buy a stonk. Unfortunalty this isnt suported")
}

pub(crate) fn sell_responce(_: &Message) -> String {
    String::from("It looks like you want to sell a stonk. Unfortunalty this isnt suported")
}
