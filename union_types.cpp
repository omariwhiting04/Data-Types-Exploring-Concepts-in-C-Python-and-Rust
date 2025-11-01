#include <iostream>
using namespace std;
union Number { int i; float f; };
int main() {
    Number n;
    n.i = 5;
    cout << n.i << endl;
    n.f = 3.14;
    cout << n.f << endl;
    return 0;
}
