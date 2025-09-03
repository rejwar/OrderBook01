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


#[derive(Debug, Default)]

struct OrderBook{
    bids: Vec<Order>,
    asks: Vec<Order>,
}


impl OrderBook {
    fn new() -> Self{
        Self { bids: Vec::new(), asks: Vec::new()}
    }

    fn add_order (&mut self , order: Order) {
        match order.side {
            OrderSide::Bid => self.bids.push(order),
            OrderSide::Ask => self.asks.push(order),
        }
    }
}

fn main() {

    let mut ob = OrderBook::new();
    let buy = Order { id: 1 , side: OrderSide::Bid , price: 100 , quantity: 10};
    let sell = Order { id: 2 , side: OrderSide::Ask , price: 101 , quantity: 5};
    println!("{:?}", buy);
    println!("{:?}", sell);

    println!("Orderbook {:?}", ob);
}