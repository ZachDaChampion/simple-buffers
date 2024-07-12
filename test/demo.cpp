#include <iostream>

#include "test.hpp"

using namespace std;
// using namespace simplebuffers;
// using namespace simplebuffers_test;

int main() {
    simplebuffers_test::MoveToEntryWriter move_to_entry_array[] = {
        simplebuffers_test::MoveToEntryWriter(simplebuffers_test::RobotJoint::J_0, 45, 100),
        simplebuffers_test::MoveToEntryWriter(simplebuffers_test::RobotJoint::J_5, 60, 200)};
    simplebuffers_test::RobotJoint joints[] = {simplebuffers_test::RobotJoint::J_1,
                                               simplebuffers_test::RobotJoint::J_2,
                                               simplebuffers_test::RobotJoint::J_3};
    auto move_to_writer = simplebuffers_test::MoveToWriter(
        simplebuffers::ListWriter<simplebuffers_test::MoveToEntryWriter>(move_to_entry_array, 2));
    simplebuffers_test::RequestWriter req_writer(
        12, simplebuffers::ListWriter<simplebuffers_test::RobotJoint>(joints, 3),
        simplebuffers_test::RequestWriter::PayloadWriter::move_to(&move_to_writer));

    uint8_t buffer[512] = {0};
    int written = req_writer.write(buffer, sizeof(buffer));
    cout << "Write result: " << written << endl;

    // Print all written bytes as hex.
    cout << hex;
    for (int i = 0; i < written; i++) {
        cout << +buffer[i] << " ";
    }
    cout << dec << endl;

    // Deserialize the data and print it.
    auto req_reader = simplebuffers_test::RequestReader(buffer);
    cout << "ID: " << req_reader.id() << endl;
    cout << "Entry 1 angle: " << req_reader.payload().move_to().joints()[1].angle() << endl;

    cout << "Array: " << static_cast<int>(req_reader.enm_array()[0]) << " "
         << static_cast<int>(req_reader.enm_array()[1]) << " "
         << static_cast<int>(req_reader.enm_array()[2]) << endl;
}