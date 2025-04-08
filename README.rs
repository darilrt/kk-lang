A simple interpreter for a interpreted lisp-like programming language.

Only the minimal features required to run the following example have been implemented.

### Examples

```scheme
(count i from 0 to 10 (
    (if (eq (mod (get i) 2) 0) (
        (print (format "{} is even" (get i)))
    )
    else (
        (print (format "{} is odd" (get i)))
    ))
))
```

