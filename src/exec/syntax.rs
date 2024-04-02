



pub enum Lambda<'a> {
    ExprOr(&'a[Self]),

    Lambda(&'a str, &'a[u32]),
}

