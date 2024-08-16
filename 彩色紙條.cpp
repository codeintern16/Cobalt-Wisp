#include<bits/stdc++.h>
using namespace std;
#define int long long
signed main(){
    ios::sync_with_stdio(0);
    cin.tie(0);
    int t;
    cin>>t;
    for(int i=0;i<t;i++){




        int n,m;
        int goal[210];
        int dp[210][210];
        cin>>n>>m;
        for(int i=0;i<n;i++){
            cin>>goal[i];
        }
        for(int i=0;i<=n;i++){
            for(int j=0;j<=n;j++){
                if(i==j)dp[i][j]=1;
                else dp[i][j]=1e4;
            }
        }
        for(int i=1;i<=n;i++){
            for(int j=0;j+i<n;j++){
                int l=j,r=j+i;
                for(int k=l;k<=r;k++){
                    if(goal[l]==goal[r]){
                        dp[l][r]=min(dp[l][r],dp[l][k]+dp[k+1][r]-1);
                    }
                    else {
                        dp[l][r]=min(dp[l][r],dp[l][k]+dp[k+1][r]);
                    }
                }
                //cout<<dp[l][r]<<' ';
            }
            //cout<<'\n';
        }
        cout<<dp[0][n-1]<<'\n';








    }


}
