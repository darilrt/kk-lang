
(count i from 0 to 10 (
    (if (eq (mod (get i) 2) 0) (
        (print (format "{} is even" (get i)))
    )
    else (
        (print (format "{} is odd" (get i)))
    ))
))

