tak2: function integer(x: integer, y: integer) = {
print ".";
    if (x <= y) {
        return y;
    } else {
        return tak2(tak2(x - 1, y), tak2(y - 1, x));
    }
}


main: function void() = {
    result1: integer = tak2(20, 1);
    result2: integer = tak2(20, 10);
    result3: integer = tak2(30, 15);

    print result1;
    print result2;
    print result3;
}
