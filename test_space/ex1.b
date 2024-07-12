gcd: function integer(a: integer, b: integer) = {

    while(a != b) {
        if(a > b) {
            a = a - b;
            }
        else {
            b = b - a;
            }
    }
    return a;
}


main: function void () = {
    result1: integer = gcd(12, 18);
    result2: integer = gcd(42, 56);
    result3: integer = gcd(1071, 462);

    print result1;
    print result2;
    print result3;
}
