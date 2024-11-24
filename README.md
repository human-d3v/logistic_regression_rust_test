Obviously, Rust isn't exactly the best medium for runnning logistic regression.

A couple hundred lines of rust that does the equivalent of the following in R:

```{R}
pacman::p_load('dplyr')

path <- './data/boston.csv'
dat <- read.csv(path)

median <- median(dat$crim)

dat$lr_crim <- dat %>% mutate(lr_crim = ifelse(crim > median, 1, 0))

model <- dat %>% glm(lr_crim ~ ., family = binomial(link = 'logit'))
```

But there is something to be said about knowing everything about your code,
implemented in Rust's type system.

To run this model:
```{bash}
git clone "https://github.com/human-d3v/logistic_regression_rust_test"
cd logistic_regression_rust_test

cargo run 
```
