#include <bits/stdc++.h>
using namespace std;


#define sz(x) (int)(x).size()
#define rep(i, a, b) for(int i = a; i < (b); ++i)
#define nl "\n"
#define nf endl
#define ll long long
#define uint unsigned int
#define pb push_back
#define _ << " " <<
#define mod 1'000'000'007
#define mp make_pair
#define trav(a, x) for (auto& a : x)
#define all(x) x.begin(), x.end()
typedef vector<int> vi;
typedef pair<ll, ll> pll;
typedef pair<int, int> pii;
typedef multiset<ll> mll;
typedef pair<double, double> pdd;
typedef array<ll, 4> tupl;
ll INF = 1'000'000'007;

ll n, t, s, m, sum, d;


int main() {
    ios::sync_with_stdio(0);
    cin.tie(0);

    string line;

    cin >> line;
    vi ints;
    trav(c, line) {
        ints.push_back(c - 'A');
    }

    int rot1 = 0, rot2 = 0;
    for (auto i = 0; i<ints.size(); i++) {
        if (i < ints.size()/2) {
            rot1 += ints[i];
        } else {
            rot2 += ints[i];
        }
    }

    for (auto i = 0; i<ints.size(); i++) {
        if (i < ints.size()/2) {
            ints[i] = (ints[i] + rot1) % ('Z' - 'A' + 1);
        } else {
            ints[i] = (ints[i] + rot2) % ('Z' - 'A' + 1);
        }
    }

    for (auto i = 0; i<ints.size()/2; i++) {
        char out = 'A' + ((ints[i] + ints[i + ints.size()/2]) % ('Z' - 'A' + 1));
        cout << out;
    }
    cout << nl;
}