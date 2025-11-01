#include <iostream>
#include <tuple>
using namespace std;
int main() {
    tuple<string,int,float> person("Alice",20,3.8);
    cout << get<0>(person) << " " << get<1>(person) << " " << get<2>(person) << endl;
    return 0;
}
