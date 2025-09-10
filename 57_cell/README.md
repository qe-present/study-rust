# cell
1. 实现了Copy
get方法获取其中的值
2. 实现了Default
take方法获取其中的值，并重置为默认值
3. 针对所有类型
relace：替换当前内部值，并返回旧值
into_inner：获取内部值，消耗self
set：设置内部值,丢弃旧值