# primenumbers
A naive implementation of a primenumber generator.

Uncomment the lines to print all or one primenumber.

When I changed the loop to only test uneven numbers
running time for 2000000 (2 million) entries was the
same. So this is a good example of premature optimization
where the compiler does optimization on my behalf.

The change was

-    for x in (3..limit) {
+    for x in (3..limit).step_by(2) {

But no change in running time.
