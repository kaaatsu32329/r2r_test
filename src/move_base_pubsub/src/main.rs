use futures::{executor::LocalPool, future, task::LocalSpawnExt, StreamExt};
use r2r::{
    geometry_msgs::{self, msg::Vector3},
    Context, Node, QosProfile,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = Context::create()?;
    let mut node = Node::create(ctx, "move_base", "")?;

    let publisher =
        node.create_publisher::<geometry_msgs::msg::Twist>("/cmd_vel", QosProfile::default())?;
    let subscriber =
        node.subscribe::<geometry_msgs::msg::Twist>("/cmd_vel", QosProfile::default())?;

    let mut timer = node.create_wall_timer(std::time::Duration::from_millis(1000))?;

    let mut pool = LocalPool::new();
    let spawner = pool.spawner();

    let twist_msg = geometry_msgs::msg::Twist {
        linear: Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        },
        angular: Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        },
    };

    spawner.spawn_local(async move {
        subscriber
            .for_each(|msg| {
                println!("[message]: {:?}", msg);
                future::ready(())
            })
            .await
    })?;

    spawner.spawn_local(async move {
        loop {
            timer.tick().await.unwrap();
            publisher.publish(&twist_msg).unwrap();
        }
    })?;

    loop {
        println!("Aaaaaaaaaaaah");
        node.spin_once(std::time::Duration::from_millis(1000));
        pool.run_until_stalled();
    }
}
