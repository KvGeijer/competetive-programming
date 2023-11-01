#include <bits/stdc++.h>
using namespace std;

#define sz(x) (int)(x).size()
#define rep(i, a, b) for (int i = a; i < (b); ++i)
#define nl "\n"
#define nf endl
#define ll long long
#define uint unsigned int
#define pb push_back
#define _ << " " <<
#define mod 1'000'000'007
#define mp make_pair
#define trav(a, x) for (auto &a : x)
#define all(x) x.begin(), x.end()
typedef vector<int> vi;
typedef pair<ll, ll> pll;
typedef pair<int, int> pii;
typedef multiset<ll> mll;
typedef pair<double, double> pdd;
typedef array<ll, 4> tupl;
ll INF = 1'000'000'007;

ll n, t, s, m, sum, d;

ll pmod(ll val) {
  ll result = val % mod;
  return result >= 0 ? result : result + mod;
}

vector<ll> phash(string str, int size) {
  vector<ll> postfix;
  ll base = 64;

  ll pow = 1;
  ll h = 0;
  for (int ind = str.length() - 1; ind >= 0; ind -= 1) {
    // update the variables
    h *= base;
    h += str[ind] - 'A';
    h = pmod(h);

    if (ind <= str.length() - size) {
      // We have passed where we will insert things
      h += str[ind];
      postfix.push_back(h);
      h -= str[ind];

      // Also remove old term
      h -= (str[ind + size - 1] - 'A') * pow;
      h = pmod(h);

    } else {
      pow *= base;
      pow = pmod(pow);
    }
  }
  return postfix;
}

int main() {
  ios::sync_with_stdio(0);
  cin.tie(0);

  string needle;
  string haystack;

  while (getline(cin, needle) && getline(cin, haystack)) {
    int len = needle.length();
    vector<ll> postfix_hashes = phash(haystack, len);
    ll needle_hash = phash(needle, len)[0];

    bool first = true;

    for (int ind = postfix_hashes.size() - 1; ind >= 0; ind -= 1) {
      if (postfix_hashes[ind] == needle_hash) {
        int start_ind = haystack.length() - len - ind;
        if (first) {
          first = false;
          cout << start_ind;
        } else {
          cout << " " << start_ind;
        }
      }
    }
    cout << nl;
  }
}
