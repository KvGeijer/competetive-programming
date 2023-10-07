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
    cin >> n >> m;

    vector<ll> p;
    p.push_back(m+1);
    ll largest = m+1;

    ll repeating = 0;
    ll layer = 1;
    while (largest * m +1 <= n)
    {
        largest = largest * m +1;
        p.push_back(largest);

        layer += 1;
        if (layer == m) {
            repeating = n - largest;
            n = largest;
            break;
            // Just repeating ones for the rest!
        }
    }

    sort(p.begin(), p.end(), greater<ll>());

    ll ans = repeating;
    ll len = n;
    for (ll div : p)
    {
        ll ones = len / div;
        ans += ones;
        len -= ones;
    }
    cout << ans << nl;
}