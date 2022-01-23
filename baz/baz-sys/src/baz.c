#include <bar/bar.h>
#include <baz/baz.h>
#include <stdio.h>

void baz_say(void) {
    bar_say();
    printf("Hello, My name is Baz!\n");
}
