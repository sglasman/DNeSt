First major hurdle: `advance_through_name` is blowing up when attempting to prove termination. I'm using a 16-bit buffer and attempting to bound the number of labels at 5 (even though this isn't realistic.)

Trying shrinking the buffer?