#include <iostream>
#include <map>
#include <string>
using namespace std;
int main() {
    map<string,int> scores;
    scores["Alice"]=90;
    scores["Bob"]=85;
    for(auto &p: scores) cout<<p.first<<" "<<p.second<<endl;
    return 0;
}
