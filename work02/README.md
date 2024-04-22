作业二  
json web token(jwt) 在用户验证领域经常被用到。请构建一个 CLI 来为给定 sub/aud/exp/… 生成一个 jwt。要求生成的 jwt 可以通过 jwt.io 的验证。  
CLI：  
rcli jwt sign --sub acme --aud device1 --exp 14d  
rcli jwt verify -t
