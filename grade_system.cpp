#include <iostream>
#include <vector>
#include <string>
using namespace std;

string grade_category(int g) {
    if (g < 50) return "Fail";
    else if (g < 70) return "Pass";
    else if (g < 80) return "Credit";
    else if (g < 90) return "Distinction";
    else return "High Distinction";
}

int main() {
    vector<int> grades = {45, 67, 72, 85, 93, 58, 79, 88, 100};
    int fail=0, pass=0, credit=0, dist=0, high=0;
    for (int g : grades) {
        string cat = grade_category(g);
        cout << g << " -> " << cat << endl;
        if (cat == "Fail") fail++;
        else if (cat == "Pass") pass++;
        else if (cat == "Credit") credit++;
        else if (cat == "Distinction") dist++;
        else high++;
    }
    cout << "Summary: F=" << fail << " P=" << pass << " C=" << credit
         << " D=" << dist << " HD=" << high << endl;
    return 0;
}
