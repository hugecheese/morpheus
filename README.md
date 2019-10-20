# Morpheus
WIP completely broken client for Matrix.

# Matrix Spec TODO
- Fix `displayname` vs `display_name`
- Make most booleans required (`false` is usually correct when field is absent)
- Make more things required (servers will commonly serve empty things as `[]`/`{}` anyway). Another example, `m.typing::user_ids` sets the array to required, even if there may be no elements, but this is not done most of the time.
- Make `room_version` required
- Question: why is the `state_key` one level back in room events?
- From `join` to `join` == changing display name?? I feel like there is a more understanble way to present an event that changes someone's display name without some join-but-not-really-a-join solution.
- Don't mark `url` in `m.image`/`m.file` as required if it's not always required
- Use `height`/`width` rather than `h`/`w`
- Encryption should be ENTIRELY SEPERATE, not mixed in with each event individually
- Consolidate all thumbnail info into seperate `Thumbnail` type, instead of repeating the `thumbnail_` prefix for several fields.
- Consistently either "redefine" or "don't redefine". The `ThumbnailInfo` is defined in many different message types, but `EncryptedFile` isn't.
- Normal names (rather than super short 2-3 character params) for EncryptedFile/JWK
- Some time fields have `_ms` suffix like `rotation_period_ms`, but most don't. Inconsistent.
- Are two different encryption algorithms needed? Complicates encrypted messages
- Differentiate between signed/unsigned and 32/64 bits for integers
- Explicitly mark `m.room.message.feedback` as deprecated
- Specify all the enum variants for `RoomVersionsCapability::available`
- Why `m.room_key_request` instead of `m.room.key_request`? inconsistent with most events
- Type names should either be PascalCase or have spaces between each word, not a mix of both
- Make all type names 100% unambiguous: There is Event, EventContent (which isn't actually the content of the previous Event), unnamed-event-content (this time for the actual Event type), State, StateEvent (which IS the event for the State), RoomEvent, RoomEventContent, ToDevice, Event (for ToDevice only), etc. These names are inconsistent. Some of them have a `Content` suffix, others do not.
- Redaction shouldn't be determined based off the absense/presence of a field, there should just be a boolean called `redacted`, or maybe some enum of the state of a given message. Determing state based off absense of JSON fields makes client implementations really clunky.
- The specification shouldn't mark things as `Required` when they are [supposedly optional](https://github.com/matrix-org/synapse/issues/6225). If they are required ONLY for sending, then it should be specified as such.
- Question: the `ToDevice` event will specify the `type`, but unlike most other events, the `EventContent` doesn't depend on any type. Does ToDevice need to specify the type? Or should the content be in a different format that varies based off the type?
- Document `m.accepted_terms`
- Specification of the fields on `m.direct` is incomplete/absent
- Undocumented `m.push_rules::device`?
- Why two possible types per field? `PushRule::actions` is weird to implement and seems to be lacking in documentation.
- Inconsistent specification of `m.receipt`. Normally, any maps of ids to objects are specified as `{string: object}`, but for `m.receipt`, we oddly specify the name of the key as the type of key (which is not the name). Unclear and inconsistent with all other map fields.
- `m.receipt` overall has a very strange and non-obvious layout, and all the fields are optional for no apparent reason.
- Inconsistent naming: `ts` vs `timestamp`
- `m.room.avatar` event but also `avatar_url`? why not just one of the two
- `m.room.third_party_invite::PublicKeys` should be renamed to `PublicKey`. It only has one key.
- Deprecate `m.room.third_party_invite::public_key`. Also, why **Required.** if it only exists for backwards compat?
- Undocumented `m.room.related_groups`
- Undocumented `m.room.plumbing`
- Question: why is so much metadata preserved after redaction? Why isn't it just marked as some generic "redacted" event?
- The `UnsignedData` inside `State Event` can contain the `prev_content` field. However, it is not documented as such. Only the `/_matrix/client/r0/pushers/set` endpoint documents `UnsignedData` to contain a `prev_content` field.
- `EventContent` has an `unsigned` field, and `UnsignedData` has a `prev_content` field. Very annoying to implement in a strongly typed language.
- The spec states that `Timeline` has an array of `RoomEvent`'s, however, many of these events contain the `state_key` field, which are only present in `State Event`'s. Either the spec is wrong or synapse.

# License
Morpheus is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License, version 3, as published by the Free Software Foundation.

Network use is distribution. If you use this code in an application which runs on a network, you must release the source code of said application under the same license.
