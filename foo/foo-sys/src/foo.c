#include <foo/foo.h>
#include <stdio.h>

void foo_say(void) {
    bar_say();
    baz_say();
    printf("Hello, My name is Foo!\n");
}
