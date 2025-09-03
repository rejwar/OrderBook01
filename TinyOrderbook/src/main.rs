#[derive(Debug)]

enum OrderSide {
    Bid,
    Ask,
}

#[derive(Debug)]
struct Order {
    id: u64,
    side: OrderSide,
    price: u64,
    quantity: u64,
}


fn main() {
    let buy = Order { id: 1 , side: OrderSide::Bid , price: 100 , quantity: 10};
    let sell = Order { id: 2 , side: OrderSide::Ask , price: 101 , quantity: 5};
    println!("{:?}", buy);
    println!("{:?}", sell);
}