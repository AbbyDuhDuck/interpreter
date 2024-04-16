
#[derive(Debug, Clone)]
pub enum Lambda<'a> {
    LambdaOr(&'a[Self]),
    Lambda(&'a str, &'a[u32]),

    GetExpr(u32, &'a Self),
    
    Eval,
    EvalAs(&'a str),
    EvalToken,
}

impl std::fmt::Display for Lambda<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ ")?;
        
        match self {
            Lambda::LambdaOr(lambdas) => {
                write!(f, "{}", lambdas
                    .iter()
                    .map(|lambda| format!("{{ {lambda} }}"))
                    .collect::<Vec<String>>()
                    .join(" || "))
            },
            Lambda::Lambda(lambda, args) => {
                write!(f, "{lambda} ")?;
                write!(f, "{} ", args.iter().map(|arg| format!("${arg}")).collect::<Vec<String>>().join(" "))
            }, 
            Lambda::GetExpr(arg, lambda) => write!(f, "with &{arg} {lambda} "),
            Lambda::Eval => write!(f, "EVAL "),
            Lambda::EvalAs(lambda) => write!(f, "{lambda} "), 
            Lambda::EvalToken => write!(f, "EVAL_TOKEN "),
        }?;

        write!(f, "}}")
    }
}

impl<'a> Into<OwnedLambda> for Lambda<'a> {
    fn into(self) -> OwnedLambda {
        match self {
            Lambda::LambdaOr(lambdas) => {
                let lambdas: Vec<OwnedLambda> = lambdas.iter().map(|l| l.into()).collect();
                OwnedLambda::LambdaOr(lambdas)
            }
            Lambda::Lambda(name, args) => OwnedLambda::Lambda(name.to_string(), args.to_vec()),
            Lambda::GetExpr(id, lambda) => OwnedLambda::GetExpr(id, Box::new(lambda.into())),
            Lambda::Eval => OwnedLambda::Eval,
            Lambda::EvalAs(name) => OwnedLambda::EvalAs(name.to_string()),
            Lambda::EvalToken => OwnedLambda::EvalToken,
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum OwnedLambda {
    LambdaOr(Vec<OwnedLambda>),
    Lambda(String, Vec<u32>),
    GetExpr(u32, Box<OwnedLambda>),
    
    Eval,
    EvalAs(String),
    EvalToken,
}

impl std::fmt::Display for OwnedLambda {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ ")?;
        
        match self {
            OwnedLambda::LambdaOr(lambdas) => {
                write!(f, "{}", lambdas
                    .iter()
                    .map(|lambda| format!("{{ {lambda} }}"))
                    .collect::<Vec<String>>()
                    .join(" || "))
            },
            OwnedLambda::Lambda(lambda, args) => {
                write!(f, "{lambda} ")?;
                write!(f, "{} ", args.iter().map(|arg| format!("${arg}")).collect::<Vec<String>>().join(" "))
            }, 
            OwnedLambda::GetExpr(arg, lambda) => write!(f, "with &{arg} {lambda} "),
            OwnedLambda::Eval => write!(f, "EVAL "),
            OwnedLambda::EvalAs(lambda) => write!(f, "{lambda} "), 
            OwnedLambda::EvalToken => write!(f, "EVAL_TOKEN "),
        }?;

        write!(f, "}}")
    }
}

impl<'a> From<&'a Lambda<'a>> for OwnedLambda {
    fn from(lambda: &'a Lambda<'a>) -> Self {
        match lambda {
            Lambda::LambdaOr(lambdas) => {
                let lambdas: Vec<OwnedLambda> = lambdas.iter().map(|l| l.into()).collect();
                OwnedLambda::LambdaOr(lambdas)
            }
            Lambda::Lambda(name, args) => OwnedLambda::Lambda(name.to_string(), args.to_vec()),
            Lambda::GetExpr(id, lambda) => OwnedLambda::GetExpr(*id, Box::new(lambda.into())),
            Lambda::Eval => OwnedLambda::Eval,
            Lambda::EvalAs(name) => OwnedLambda::EvalAs(name.to_string()),
            Lambda::EvalToken => OwnedLambda::EvalToken,
        }
    }
}

impl<'a> From<&&'a Lambda<'a>> for OwnedLambda {
    fn from(lambda: &&'a Lambda<'a>) -> Self {
        let borrowed_lambda: &'a Lambda<'a> = *lambda; // Dereference once to get `&Lambda`
        From::from(borrowed_lambda) // Convert borrowed Lambda to OwnedLambda
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into_owned_lambda_lambda() {
        // Create a Lambda::Lambda variant
        let lambda = Lambda::Lambda("test", &[1, 2, 3]);

        // Convert Lambda into OwnedLambda
        let owned_lambda: OwnedLambda = lambda.into();

        // Assert that the conversion produced the expected OwnedLambda variant
        assert_eq!(
            owned_lambda,
            OwnedLambda::Lambda("test".to_string(), vec![1, 2, 3])
        );
    }

    #[test]
    fn test_into_owned_lambda_lambda_or() {
        // Create a Lambda::LambdaOr variant
        let lambda_or = Lambda::LambdaOr(&[
            Lambda::Lambda("test1", &[1, 2]),
            Lambda::Lambda("test2", &[3, 4]),
        ]);

        // Convert LambdaOr into OwnedLambda
        let owned_lambda: OwnedLambda = lambda_or.into();

        // Assert that the conversion produced the expected OwnedLambda variant
        assert_eq!(
            owned_lambda,
            OwnedLambda::LambdaOr(vec![
                OwnedLambda::Lambda("test1".to_string(), vec![1, 2]),
                OwnedLambda::Lambda("test2".to_string(), vec![3, 4])
            ])
        );
    }

    #[test]
    fn test_into_owned_lambda_get_expr() {
        // Create a Lambda::Lambda variant
        let lambda = Lambda::Lambda("test", &[1, 2, 3]);
        // Create a Lambda::GetExpr variant
        let lambda_get_expr = Lambda::GetExpr(42, &lambda);

        // Convert GetExpr into OwnedLambda
        let owned_lambda: OwnedLambda = lambda_get_expr.into();

        // Assert that the conversion produced the expected OwnedLambda variant
        assert_eq!(
            owned_lambda,
            OwnedLambda::GetExpr(
                42,
                Box::new(OwnedLambda::Lambda("test".to_string(), vec![1, 2, 3]))
            )
        );
    }

    #[test]
    fn test_all_lambda_variants_into_owned_lambda() {
        // Lambda::LambdaOr variant
        let lambda_or = Lambda::LambdaOr(&[
            Lambda::Lambda("test1", &[1, 2]),
            Lambda::Lambda("test2", &[3, 4]),
        ]);
        let owned_lambda_or: OwnedLambda = lambda_or.into();
        assert_eq!(
            owned_lambda_or,
            OwnedLambda::LambdaOr(vec![
                OwnedLambda::Lambda("test1".to_string(), vec![1, 2]),
                OwnedLambda::Lambda("test2".to_string(), vec![3, 4])
            ])
        );
    
        // Lambda::Lambda variant
        let lambda_lambda = Lambda::Lambda("test", &[5, 6]);
        let owned_lambda_lambda: OwnedLambda = lambda_lambda.into();
        assert_eq!(
            owned_lambda_lambda,
            OwnedLambda::Lambda("test".to_string(), vec![5, 6])
        );
    
        // Lambda::GetExpr variant
        let lambda_lambda = Lambda::Lambda("test", &[5, 6]);
        let lambda_get_expr = Lambda::GetExpr(42, &lambda_lambda);
        let owned_lambda_get_expr: OwnedLambda = lambda_get_expr.into();
        assert_eq!(
            owned_lambda_get_expr,
            OwnedLambda::GetExpr(
                42,
                Box::new(OwnedLambda::Lambda("test".to_string(), vec![5, 6]))
            )
        );
    
        // Lambda::Eval variant
        let lambda_eval = Lambda::Eval;
        let owned_lambda_eval: OwnedLambda = lambda_eval.into();
        assert_eq!(owned_lambda_eval, OwnedLambda::Eval);
    
        // Lambda::EvalAs variant
        let lambda_eval_as = Lambda::EvalAs("test");
        let owned_lambda_eval_as: OwnedLambda = lambda_eval_as.into();
        assert_eq!(
            owned_lambda_eval_as,
            OwnedLambda::EvalAs("test".to_string())
        );
    
        // Lambda::EvalToken variant
        let lambda_eval_token = Lambda::EvalToken;
        let owned_lambda_eval_token: OwnedLambda = lambda_eval_token.into();
        assert_eq!(owned_lambda_eval_token, OwnedLambda::EvalToken);
    }
    

}
