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

    cin >> n;
    vi trees;
    for (int i = 0; i < n; i++) {
        cin >> t;
        trees.push_back(t);
    }

    sort(all(trees), greater<int>());
    
    int i=1;
    ll max = 0;
    trav(tree, trees) {
        i += 1;
        ll done = tree + i;
        if (done > max) {
            max = done;
        }
    }

    cout << max;
}