There is a natural point at which,
we can return the memory our String,
needs to the allocator: when s goes,
out of scope. When a variable goes out,
of scope, Rust calls a special function for us. 
This function is called drop, and it’s where the
author of String can put the code to return
the memory. Rust calls drop automatically at the closing curly bracket.