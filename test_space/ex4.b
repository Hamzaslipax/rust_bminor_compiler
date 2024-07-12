sum_of_first_n: function integer(n: integer) = {
    sum: integer = 0;
    i: integer = 1;

    while (i <= n) {
        sum = sum + i;
        i = i + 1;
    }

    return sum;
}

main: function void() = {
    n: integer = 5;

    result: integer = sum_of_first_n(n);
    print "Sum of the first natural numbers is: ";
    print result;
}
