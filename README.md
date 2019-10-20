# Morpheus
WIP completely broken client for Matrix.

# Matrix Spec TODO
- Fix `displayname` vs `display_name`
- Make most booleans required (`false` is usually correct when field is absent)
- Make more things required (servers will commonly empty things as `[]`/`{}`)
- Make `room_version` required
- Question: why is the `state_key` one level back in room events?
- From `join` to `join` == changing display name???
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

# License
Morpheus is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License, version 3, as published by the Free Software Foundation.

Network use is distribution. If you use this code in an application which runs on a network, you must release the source code of said application under the same license.
