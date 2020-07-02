use std::time::Duration;
use xactor::*;

#[xactor::main]
async fn main() -> std::io::Result<()> {
    let parent_addr = ParentActor::default().start().await.unwrap();
    parent_addr.wait_for_stop().await;
    Ok(())
}

#[derive(Clone)]
pub struct ParentActor {
    children: Vec<Addr<ChildActor>>,
}

impl Default for ParentActor {
    fn default() -> ParentActor {
        ParentActor {
            children: Vec::new(),
        }
    }
}

#[message]
struct Die;

#[async_trait::async_trait]
impl Handler<Die> for ParentActor {
    async fn handle(&mut self, ctx: &Context<Self>, _msg: Die) {
        println!("Die Message Recieved");
        ctx.stop(None);
    }
}

#[message]
struct ResetChildren;

#[async_trait::async_trait]
impl Handler<ResetChildren> for ParentActor {
    async fn handle(&mut self, _ctx: &Context<Self>, _msg: ResetChildren) {
        println!("Resetting Children");
        self.children = Vec::new();
    }
}

#[message]
struct InitializeChildren;

#[async_trait::async_trait]
impl Handler<InitializeChildren> for ParentActor {
    async fn handle(&mut self, _ctx: &Context<Self>, _msg: InitializeChildren) {
        println!("Initializing Children");
        let dummy_ids: Vec<i32> = vec![1, 2, 3, 4, 5];
        let children_addr_vec = dummy_ids
            .into_iter()
            .map(|id| async move { ChildActor::new(id).start().await.unwrap() });
        let children_addr_vec = futures::future::join_all(children_addr_vec).await;

        self.children = children_addr_vec;
    }
}

#[async_trait::async_trait]
impl Actor for ParentActor {
    async fn started(&mut self, ctx: &Context<Self>) -> Result<()> {
        println!("Parent Started");
        let _ = ctx.address().send(InitializeChildren);
        ctx.send_later(ResetChildren, Duration::from_secs(2));
        ctx.send_later(Die, Duration::from_secs(4));

        Ok(())
    }

    async fn stopped(&mut self, _ctx: &Context<Self>) {
        println!("Parent Stopped");
    }
}

#[derive(Clone)]
pub struct ChildActor {
    id: i32,
}

impl ChildActor {
    fn new(id: i32) -> ChildActor {
        ChildActor { id }
    }
}

#[async_trait::async_trait]
impl Actor for ChildActor {
    async fn started(&mut self, _ctx: &Context<Self>) -> Result<()> {
        println!("Child Started, id: {:?}", self.id);
        Ok(())
    }

    async fn stopped(&mut self, _ctx: &Context<Self>) {
        println!("Child Stopped, id: {:?}", self.id);
    }
}
