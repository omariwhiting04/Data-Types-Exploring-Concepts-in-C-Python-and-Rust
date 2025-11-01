#include <iostream>
#include <string>
using namespace std;
struct Student { string name; int id; };
int main() {
    Student s = {"Alice", 1};
    cout << s.name << " " << s.id << endl;
    return 0;
}
