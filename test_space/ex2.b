factorial: function integer (n: integer) = {
    result: integer;
    if (n >= 1) {
        result = factorial(n - 1);
        return result * n;
    } else {
        return 1;
    }
}

main: function void () = {
    m: integer;
    result: integer = factorial(10);
    print "the result is:";
    print result;
}
