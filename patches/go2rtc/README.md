# Pending go2rtc fixes

`0001-close-rejected-http-responses.patch` closes non-200 HTTP source responses
and adds a race-tested regression for unfinished 503/404/500 response bodies.
It applies to submodule revision `6986a1b3f122651b318a957534da44bb4c65e271`.
The reconnect qualification reproduced accumulating descriptors before this fix.

Run `sh scripts/apply-go2rtc-patches.sh` before building the current go2rtc
source. An optional first argument selects another source directory. The script
accepts pristine or already-applied source, preserves unrelated edits and fails
before writing when the patch does not apply. It also works on source copies
without Git metadata. Docker and CI call it explicitly.

No submodule commit or parent pointer change is required for this delivery.
When the fix and regression are incorporated into the pinned upstream source,
remove the pending patch and its build hooks in the same reviewed update.
Historical upgrade fixtures intentionally build their original unpatched pin.
