<?php

declare(strict_types=1);

function minimum_divided_by_minus_one(): float
{
    return (-9223372036854775807 - 1) / -1;
}


/**
 * @return 4611686018427387904
 */
function minimum_divided_by_minus_two(): int
{
    return (-9223372036854775807 - 1) / -2;
}
