#include <iostream>

#include "test.hpp"

using namespace std;
using namespace simplebuffers;
using namespace simplebuffers_test;

int main() {
    MoveToEntryWriter move_to_entry_array[] = {MoveToEntryWriter(RobotJoint::J_0, 45, 100),
                                               MoveToEntryWriter(RobotJoint::J_5, 60, 200)};

    RobotJoint joints[] = {RobotJoint::J_1, RobotJoint::J_2, RobotJoint::J_3};

    auto move_to_writer = MoveToWriter(ListWriter<MoveToEntryWriter>(move_to_entry_array, 2));

    auto req_writer = RequestWriter(12, ListWriter<RobotJoint>(joints, 3),
                                    RequestWriter::PayloadWriter::move_to(&move_to_writer));

    uint8_t buffer[512] = {0};
    int written = req_writer.write(buffer, sizeof(buffer));
    cout << "Write result: " << written << endl << hex;
    for (int i = 0; i < written; i++) {
        cout << +buffer[i] << " ";
    }
    cout << dec << endl;

    // Deserialize the data and print it.
    auto req_reader = RequestReader(buffer);
    cout << "ID: " << req_reader.id() << endl;
    cout << "Entry 1 angle: " << req_reader.payload().move_to().joints()[1].angle() << endl;
    cout << "Array: " << static_cast<int>(req_reader.enm_array()[0]) << " "
         << static_cast<int>(req_reader.enm_array()[1]) << " "
         << static_cast<int>(req_reader.enm_array()[2]) << endl;
}