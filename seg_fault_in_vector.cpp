#include <vector>
#include <memory>
#include <iostream>

using namespace std;

int main() {
    vector<unique_ptr<int>> v;
    v.push_back(make_unique<int>(5));

    cout << *v[0] << endl;

    // C++ happily allows this.
    auto pointer_to_5 = move(v[0]);
    cout << *pointer_to_5 << endl;

    // Seg fault.
    cout << *v[0] << endl;
}
