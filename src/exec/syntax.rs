



pub enum Lambda<'a> {
    LambdaOr(&'a[Self]),
    Lambda(&'a str, &'a[u32]),

    GetExpr(u32, &'a Self),
    
    Eval,
    EvalAs(&'a str),
    EvalToken,
}

