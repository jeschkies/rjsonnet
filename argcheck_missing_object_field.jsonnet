#! inc : number -> number
local inc = function(x) x + 1;

#! inc_complex : { a: number } -> number
local inc_complex = function(x) x.a + 1;

{
  a: inc_complex({a: 1}),
  b: inc_complex({b: 3}),

  c: inc('3'),
}


