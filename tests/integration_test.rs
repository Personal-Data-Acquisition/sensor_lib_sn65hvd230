/*
 * Authors: Jake G, <Add you name here>
 * Date: 2023
 * FileName: integration_test.rs
 */

use sensor_lib_sn65hvd230::add;


//Simple integration test for providing an add function.
#[test]
fn it_adds_usize() {
    let x: usize = 25;
    let y: usize = 50;
    assert_eq!(75, add(x, y) );
}
