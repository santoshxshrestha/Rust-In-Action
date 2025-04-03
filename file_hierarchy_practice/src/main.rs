// ========================
// Main Function
// ========================
fn main() {
    // Import necessary items from modules
    use file_hierarchy_practice::*;
    use file_hierarchy_practice::shapes::circle::Circle;
    use file_hierarchy_practice::shapes::rectangle::Rectangle;
    use file_hierarchy_practice::shapes::triangle::Triangle;
    use file_hierarchy_practice::drawing::{draw_shape, draw_multiple};
    use file_hierarchy_practice::utils::{print_message, log_info, log_error, simulate_delay};

    // Create instances of shapes using constructors
    let circle1 = Circle::new(7.0);
    let rectangle1 = Rectangle::new(10.0, 5.0);
    let triangle1 = Triangle::new(8.0, 4.0, 5.0, 6.0, 7.0);

    // Log that shapes have been created
    log_info("Primary shapes created successfully!");

    // Draw each shape individually with a separator
    println!("--- Drawing individual shapes ---");
    draw_shape(&circle1);
    println!("----------------------------");
    draw_shape(&rectangle1);
    println!("----------------------------");
    draw_shape(&triangle1);
    println!("----------------------------");

    // Create a collection of shapes for batch drawing
    let shapes_collection: Vec<&dyn drawing::Drawable> = vec![&circle1, &rectangle1, &triangle1];
    println!("--- Drawing shapes from a collection ---");
    draw_multiple(&shapes_collection);

    // Use utility function to print a completion message
    print_message("Initial drawing complete!");

    // Simulate additional work using a loop
    println!("--- Repeated Drawing Loop ---");
    for iteration in 1..=5 {
        println!("Iteration {}: Drawing the circle again", iteration);
        draw_shape(&circle1);
        println!("----------------------------");
    }

    // Create additional shapes for further practice
    let circle2 = Circle::new(3.5);
    let rectangle2 = Rectangle::new(4.0, 2.0);
    let triangle2 = Triangle::new(6.0, 3.0, 4.0, 5.0, 6.0);

    log_info("Additional shapes created for extended practice.");

    // Combine all shapes into one vector
    let more_shapes: Vec<&dyn drawing::Drawable> = vec![&circle2, &rectangle2, &triangle2];
    println!("--- Drawing additional shapes ---");
    draw_multiple(&more_shapes);

    // Simulate delay for practice purposes
    println!("--- Simulating delay ---");
    simulate_delay();

    // Final message and error simulation
    print_message("All shapes drawn. Practice complete!");
    log_error("This is a simulated error message for testing purposes.");

    // Extra: Loop to print details repeatedly for practice
    println!("--- Extra Loop for Extended Practice ---");
    for i in 1..=3 {
        println!("Extra iteration {}: Redrawing all shapes", i);
        draw_multiple(&shapes_collection);
        println!("Extra iteration {}: Redrawing additional shapes", i);
        draw_multiple(&more_shapes);
        println!("========================================");
    }
}
