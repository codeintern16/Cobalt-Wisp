#include <bits/stdc++.h>
using namespace std;
#define int long long
int dp[100010];
int ans = 0;
int a[100010];
const int mod = 1e9+7;
int n, m;
void solve(int x){


    if(x>n){
        /*for(int i=1;i<=n;i++){
            cout<<a[i]<<' ';
        }
        cout<<'\n';*/
        ans++;
        return;
    }
    if(a[x]!=0){
        if(abs(a[x]-a[x-1])>1 && x!=1)return;
        solve(x+1);
    }
    else{
        if(a[x-1]+1 <=m){
            a[x] = a[x-1] + 1;
            solve(x+1);
        }
        a[x] = a[x-1];
        solve(x+1);
        if(a[x-1]-1>0){
            a[x] = a[x-1]-1;
            solve(x+1);
        }
        a[x] = 0;
    }

}
signed main(){
    ios::sync_with_stdio(0);
    cin.tie(0);
    cout.tie(0);
    cin>>n>>m;
    for(int i=1;i<=n;i++){
        cin>>a[i];
    }
    a[n+1] = -1;
    if(a[1]!=0)solve(1);
    else{
        for(int i=1;i<=m;i++){
            a[1] = i;
            solve(1);
        }
    }
    cout<<ans;

}
