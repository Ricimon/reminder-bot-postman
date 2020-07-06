table! {
    reminders (id) {
        id -> Unsigned<Integer>,
        uid -> VarChar,

        name -> Nullable<VarChar>,

        message_id -> Unsigned<Integer>,

        channel_id -> Unsigned<Integer>,

        time -> Unsigned<Integer>,
        enabled -> Bool,

        avatar -> VarChar,
        username -> VarChar,

        interval -> Nullable<Unsigned<Integer>>,

        method -> Nullable<VarChar>,
        set_by -> Unsigned<Integer>,
        set_at -> Timestamp,
    }
}

table! {
    messages (id) {
        id -> Unsigned<Integer>,

        content ->  VarChar,

        embed_id -> Nullable<Unsigned<Integer>>,
    }
}

table! {
    embeds (id) {
        id -> Unsigned<Integer>,

        title -> VarChar,
        description -> VarChar,
        color -> Unsigned<Integer>,
    }
}

table! {
    channels (id) {
        id -> Unsigned<Integer>,
        channel -> Unsigned<BigInt>,

        name -> Nullable<VarChar>,

        nudge -> SmallInt,
        blacklisted -> Bool,

        webhook_id -> Nullable<Unsigned<BigInt>>,
        webhook_token -> Nullable<VarChar>,

        paused -> Bool,
        paused_until -> Nullable<Timestamp>,

        guild_id -> Nullable<Unsigned<Integer>>,
    }
}
