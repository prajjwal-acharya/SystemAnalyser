with open("src/lib.rs", "r") as f:
    content = f.read()

import re
resolved = re.sub(
    r"<<<<<<< Updated upstream.*?=======.*?(?:>>>>>>> Stashed changes)",
    "if handle_key_event_or_break(event, &mut app, &mut painter.layout, &collection_thread_ctrl_sender) {",
    content,
    flags=re.DOTALL
)

with open("src/lib.rs", "w") as f:
    f.write(resolved)
