fn f<S>(s: S)
where
    S: Send, /* */
    //
{
}

fn main() {
    foo(
        x, // comment 1
        // comment 2
    )
}

fn main() {
    foo(
        x, // comment 1
        // comment 2
    )
}

fn main() {
    foo(
        x, /* Comment 1 */
        /* Comment 2 */
    )
}

fn main() {
    foo(
        x, /* Comment 1 */
        /* Comment 2 */
    )
}

fn main() {
    foo(
        x, /* comment 1 */
        /* comment 2 */
    )
}
