factorial: function integer(n: integer) = {
    result: integer = 1;
    while(n > 0) {
        result = result * n;
        n = n - 1;
    }
    return result;
}

a: integer=7;

main: function void() = {

    
    x: integer;
    y: integer = 5;
    x = factorial(a + y);
    print x;
    
    if(x > y) {
        print "x is greater than y";
    } else {
        print "y is greater than or equal to x";
    }
}
