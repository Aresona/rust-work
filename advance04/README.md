## 作业截图
![](./images/advance01.jpg)
![](./images/advance02.jpg)

### 回答链上随机数与链下随机数的区别
1. sp_io::offchain::random_seed 通过链下主机环境生成一个真正的随机数．每个节点在执行时生成的都是不同的
2. T::Randomness::random_seed 生成的随机数在生成区块之前是不可预测的, 但在同一区块中是确定的，所有节点在执行同一区块时生成的随机数是相同的

