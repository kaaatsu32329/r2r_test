use r2r::{
    geometry_msgs::{self, msg::Vector3},
    Context, Node, QosProfile,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = Context::create()?;
    let mut node = Node::create(ctx, "move_base", "")?;

    let publisher =
        node.create_publisher::<geometry_msgs::msg::Twist>("cmd_vel", QosProfile::default())?;

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

    loop {
        publisher.publish(&twist_msg)?;
    }
}
