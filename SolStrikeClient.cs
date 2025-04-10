using System;
using System.Collections.Generic;
using System.Linq;
using System.Numerics;
using System.Threading.Tasks;
using Solana.Unity;
using Solana.Unity.Programs.Abstract;
using Solana.Unity.Programs.Utilities;
using Solana.Unity.Rpc;
using Solana.Unity.Rpc.Builders;
using Solana.Unity.Rpc.Core.Http;
using Solana.Unity.Rpc.Core.Sockets;
using Solana.Unity.Rpc.Types;
using Solana.Unity.Wallet;
using SolStrike;
using SolStrike.Program;
using SolStrike.Errors;
using SolStrike.Accounts;
using SolStrike.Events;
using SolStrike.Types;

namespace SolStrike
{
    namespace Accounts
    {
        public partial class ClaimableRewards
        {
            public static ulong ACCOUNT_DISCRIMINATOR => 15769378728584491768UL;
            public static ReadOnlySpan<byte> ACCOUNT_DISCRIMINATOR_BYTES => new byte[]{248, 50, 225, 101, 103, 22, 216, 218};
            public static string ACCOUNT_DISCRIMINATOR_B58 => "iWqeifhaXLm";
            public ulong Amount { get; set; }

            public static ClaimableRewards Deserialize(ReadOnlySpan<byte> _data)
            {
                int offset = 0;
                ulong accountHashValue = _data.GetU64(offset);
                offset += 8;
                if (accountHashValue != ACCOUNT_DISCRIMINATOR)
                {
                    return null;
                }

                ClaimableRewards result = new ClaimableRewards();
                result.Amount = _data.GetU64(offset);
                offset += 8;
                return result;
            }
        }

        public partial class GlobalConfig
        {
            public static ulong ACCOUNT_DISCRIMINATOR => 15686315269655627925UL;
            public static ReadOnlySpan<byte> ACCOUNT_DISCRIMINATOR_BYTES => new byte[]{149, 8, 156, 202, 160, 252, 176, 217};
            public static string ACCOUNT_DISCRIMINATOR_B58 => "Rvp9zjtEEBA";
            public ulong LamportsChipPrice { get; set; }

            public byte Bump { get; set; }

            public static GlobalConfig Deserialize(ReadOnlySpan<byte> _data)
            {
                int offset = 0;
                ulong accountHashValue = _data.GetU64(offset);
                offset += 8;
                if (accountHashValue != ACCOUNT_DISCRIMINATOR)
                {
                    return null;
                }

                GlobalConfig result = new GlobalConfig();
                result.LamportsChipPrice = _data.GetU64(offset);
                offset += 8;
                result.Bump = _data.GetU8(offset);
                offset += 1;
                return result;
            }
        }

        public partial class Treasury
        {
            public static ulong ACCOUNT_DISCRIMINATOR => 18277860573447974894UL;
            public static ReadOnlySpan<byte> ACCOUNT_DISCRIMINATOR_BYTES => new byte[]{238, 239, 123, 238, 89, 1, 168, 253};
            public static string ACCOUNT_DISCRIMINATOR_B58 => "gxyTsYaqFet";
            public byte Bump { get; set; }

            public static Treasury Deserialize(ReadOnlySpan<byte> _data)
            {
                int offset = 0;
                ulong accountHashValue = _data.GetU64(offset);
                offset += 8;
                if (accountHashValue != ACCOUNT_DISCRIMINATOR)
                {
                    return null;
                }

                Treasury result = new Treasury();
                result.Bump = _data.GetU8(offset);
                offset += 1;
                return result;
            }
        }
    }

    namespace Errors
    {
        public enum SolStrikeErrorKind : uint
        {
            Overflow = 6000U
        }
    }

    namespace Events
    {
    }

    namespace Types
    {
    }

    public partial class SolStrikeClient : TransactionalBaseClient<SolStrikeErrorKind>
    {
        public SolStrikeClient(IRpcClient rpcClient, IStreamingRpcClient streamingRpcClient, PublicKey programId = null) : base(rpcClient, streamingRpcClient, programId ?? new PublicKey(SolStrikeProgram.ID))
        {
        }

        public async Task<Solana.Unity.Programs.Models.ProgramAccountsResultWrapper<List<ClaimableRewards>>> GetClaimableRewardssAsync(string programAddress = SolStrikeProgram.ID, Commitment commitment = Commitment.Confirmed)
        {
            var list = new List<Solana.Unity.Rpc.Models.MemCmp>{new Solana.Unity.Rpc.Models.MemCmp{Bytes = ClaimableRewards.ACCOUNT_DISCRIMINATOR_B58, Offset = 0}};
            var res = await RpcClient.GetProgramAccountsAsync(programAddress, commitment, memCmpList: list);
            if (!res.WasSuccessful || !(res.Result?.Count > 0))
                return new Solana.Unity.Programs.Models.ProgramAccountsResultWrapper<List<ClaimableRewards>>(res);
            List<ClaimableRewards> resultingAccounts = new List<ClaimableRewards>(res.Result.Count);
            resultingAccounts.AddRange(res.Result.Select(result => ClaimableRewards.Deserialize(Convert.FromBase64String(result.Account.Data[0]))));
            return new Solana.Unity.Programs.Models.ProgramAccountsResultWrapper<List<ClaimableRewards>>(res, resultingAccounts);
        }

        public async Task<Solana.Unity.Programs.Models.ProgramAccountsResultWrapper<List<GlobalConfig>>> GetGlobalConfigsAsync(string programAddress = SolStrikeProgram.ID, Commitment commitment = Commitment.Confirmed)
        {
            var list = new List<Solana.Unity.Rpc.Models.MemCmp>{new Solana.Unity.Rpc.Models.MemCmp{Bytes = GlobalConfig.ACCOUNT_DISCRIMINATOR_B58, Offset = 0}};
            var res = await RpcClient.GetProgramAccountsAsync(programAddress, commitment, memCmpList: list);
            if (!res.WasSuccessful || !(res.Result?.Count > 0))
                return new Solana.Unity.Programs.Models.ProgramAccountsResultWrapper<List<GlobalConfig>>(res);
            List<GlobalConfig> resultingAccounts = new List<GlobalConfig>(res.Result.Count);
            resultingAccounts.AddRange(res.Result.Select(result => GlobalConfig.Deserialize(Convert.FromBase64String(result.Account.Data[0]))));
            return new Solana.Unity.Programs.Models.ProgramAccountsResultWrapper<List<GlobalConfig>>(res, resultingAccounts);
        }

        public async Task<Solana.Unity.Programs.Models.ProgramAccountsResultWrapper<List<Treasury>>> GetTreasurysAsync(string programAddress = SolStrikeProgram.ID, Commitment commitment = Commitment.Confirmed)
        {
            var list = new List<Solana.Unity.Rpc.Models.MemCmp>{new Solana.Unity.Rpc.Models.MemCmp{Bytes = Treasury.ACCOUNT_DISCRIMINATOR_B58, Offset = 0}};
            var res = await RpcClient.GetProgramAccountsAsync(programAddress, commitment, memCmpList: list);
            if (!res.WasSuccessful || !(res.Result?.Count > 0))
                return new Solana.Unity.Programs.Models.ProgramAccountsResultWrapper<List<Treasury>>(res);
            List<Treasury> resultingAccounts = new List<Treasury>(res.Result.Count);
            resultingAccounts.AddRange(res.Result.Select(result => Treasury.Deserialize(Convert.FromBase64String(result.Account.Data[0]))));
            return new Solana.Unity.Programs.Models.ProgramAccountsResultWrapper<List<Treasury>>(res, resultingAccounts);
        }

        public async Task<Solana.Unity.Programs.Models.AccountResultWrapper<ClaimableRewards>> GetClaimableRewardsAsync(string accountAddress, Commitment commitment = Commitment.Finalized)
        {
            var res = await RpcClient.GetAccountInfoAsync(accountAddress, commitment);
            if (!res.WasSuccessful)
                return new Solana.Unity.Programs.Models.AccountResultWrapper<ClaimableRewards>(res);
            var resultingAccount = ClaimableRewards.Deserialize(Convert.FromBase64String(res.Result.Value.Data[0]));
            return new Solana.Unity.Programs.Models.AccountResultWrapper<ClaimableRewards>(res, resultingAccount);
        }

        public async Task<Solana.Unity.Programs.Models.AccountResultWrapper<GlobalConfig>> GetGlobalConfigAsync(string accountAddress, Commitment commitment = Commitment.Finalized)
        {
            var res = await RpcClient.GetAccountInfoAsync(accountAddress, commitment);
            if (!res.WasSuccessful)
                return new Solana.Unity.Programs.Models.AccountResultWrapper<GlobalConfig>(res);
            var resultingAccount = GlobalConfig.Deserialize(Convert.FromBase64String(res.Result.Value.Data[0]));
            return new Solana.Unity.Programs.Models.AccountResultWrapper<GlobalConfig>(res, resultingAccount);
        }

        public async Task<Solana.Unity.Programs.Models.AccountResultWrapper<Treasury>> GetTreasuryAsync(string accountAddress, Commitment commitment = Commitment.Finalized)
        {
            var res = await RpcClient.GetAccountInfoAsync(accountAddress, commitment);
            if (!res.WasSuccessful)
                return new Solana.Unity.Programs.Models.AccountResultWrapper<Treasury>(res);
            var resultingAccount = Treasury.Deserialize(Convert.FromBase64String(res.Result.Value.Data[0]));
            return new Solana.Unity.Programs.Models.AccountResultWrapper<Treasury>(res, resultingAccount);
        }

        public async Task<SubscriptionState> SubscribeClaimableRewardsAsync(string accountAddress, Action<SubscriptionState, Solana.Unity.Rpc.Messages.ResponseValue<Solana.Unity.Rpc.Models.AccountInfo>, ClaimableRewards> callback, Commitment commitment = Commitment.Finalized)
        {
            SubscriptionState res = await StreamingRpcClient.SubscribeAccountInfoAsync(accountAddress, (s, e) =>
            {
                ClaimableRewards parsingResult = null;
                if (e.Value?.Data?.Count > 0)
                    parsingResult = ClaimableRewards.Deserialize(Convert.FromBase64String(e.Value.Data[0]));
                callback(s, e, parsingResult);
            }, commitment);
            return res;
        }

        public async Task<SubscriptionState> SubscribeGlobalConfigAsync(string accountAddress, Action<SubscriptionState, Solana.Unity.Rpc.Messages.ResponseValue<Solana.Unity.Rpc.Models.AccountInfo>, GlobalConfig> callback, Commitment commitment = Commitment.Finalized)
        {
            SubscriptionState res = await StreamingRpcClient.SubscribeAccountInfoAsync(accountAddress, (s, e) =>
            {
                GlobalConfig parsingResult = null;
                if (e.Value?.Data?.Count > 0)
                    parsingResult = GlobalConfig.Deserialize(Convert.FromBase64String(e.Value.Data[0]));
                callback(s, e, parsingResult);
            }, commitment);
            return res;
        }

        public async Task<SubscriptionState> SubscribeTreasuryAsync(string accountAddress, Action<SubscriptionState, Solana.Unity.Rpc.Messages.ResponseValue<Solana.Unity.Rpc.Models.AccountInfo>, Treasury> callback, Commitment commitment = Commitment.Finalized)
        {
            SubscriptionState res = await StreamingRpcClient.SubscribeAccountInfoAsync(accountAddress, (s, e) =>
            {
                Treasury parsingResult = null;
                if (e.Value?.Data?.Count > 0)
                    parsingResult = Treasury.Deserialize(Convert.FromBase64String(e.Value.Data[0]));
                callback(s, e, parsingResult);
            }, commitment);
            return res;
        }

        protected override Dictionary<uint, ProgramError<SolStrikeErrorKind>> BuildErrorsDictionary()
        {
            return new Dictionary<uint, ProgramError<SolStrikeErrorKind>>{};
        }
    }

    namespace Program
    {
        public class BuyChipWithSolAccounts
        {
            public PublicKey Buyer { get; set; }

            public PublicKey GlobalConfig { get; set; }

            public PublicKey Treasury { get; set; }

            public PublicKey ChipMint { get; set; }

            public PublicKey BuyerChipAccount { get; set; }

            public PublicKey TokenProgram { get; set; }

            public PublicKey AssociatedTokenProgram { get; set; } = new PublicKey("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");
            public PublicKey SystemProgram { get; set; } = new PublicKey("11111111111111111111111111111111");
        }

        public class ClaimChipsAccounts
        {
            public PublicKey Signer { get; set; }

            public PublicKey ClaimableRewardsAccount { get; set; }

            public PublicKey ChipMint { get; set; }

            public PublicKey Treasury { get; set; }

            public PublicKey TreasuryChipTokenAccount { get; set; }

            public PublicKey ClaimerChipAccount { get; set; }

            public PublicKey TokenProgram { get; set; }
        }

        public class InitializeAccounts
        {
            public PublicKey GlobalConfig { get; set; }

            public PublicKey ChipMint { get; set; }

            public PublicKey Treasury { get; set; }

            public PublicKey TreasuryChipTokenAccount { get; set; }

            public PublicKey Signer { get; set; }

            public PublicKey Program { get; set; } = new PublicKey("F7Dr4bH5knKjzBj8fuRJT9QGtHLyQSWTnWxYetHDnWHA");
            public PublicKey ProgramData { get; set; }

            public PublicKey TokenProgram { get; set; }

            public PublicKey AssociatedTokenProgram { get; set; } = new PublicKey("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");
            public PublicKey SystemProgram { get; set; } = new PublicKey("11111111111111111111111111111111");
        }

        public class ReserveChipsAccounts
        {
            public PublicKey Signer { get; set; }

            public PublicKey Treasury { get; set; }

            public PublicKey ChipMint { get; set; }

            public PublicKey TreasuryChipTokenAccount { get; set; }

            public PublicKey UserChipAccount { get; set; }

            public PublicKey TokenProgram { get; set; }
        }

        public class SellChipAccounts
        {
            public PublicKey Seller { get; set; }

            public PublicKey GlobalConfig { get; set; }

            public PublicKey ChipMint { get; set; }

            public PublicKey Treasury { get; set; }

            public PublicKey SellerChipAccount { get; set; }

            public PublicKey TokenProgram { get; set; }

            public PublicKey AssociatedTokenProgram { get; set; } = new PublicKey("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");
        }

        public class SetClaimableRewardsAccounts
        {
            public PublicKey Signer { get; set; }

            public PublicKey Program { get; set; } = new PublicKey("F7Dr4bH5knKjzBj8fuRJT9QGtHLyQSWTnWxYetHDnWHA");
            public PublicKey ProgramData { get; set; }

            public PublicKey FirstPlaceClaimableRewardsAccount { get; set; }

            public PublicKey FirstPlaceAuthority { get; set; }

            public PublicKey SecondPlaceClaimableRewardsAccount { get; set; }

            public PublicKey SecondPlaceAuthority { get; set; }

            public PublicKey ThirdPlaceClaimableRewardsAccount { get; set; }

            public PublicKey ThirdPlaceAuthority { get; set; }

            public PublicKey SystemProgram { get; set; } = new PublicKey("11111111111111111111111111111111");
        }

        public class UpdateSolChipPriceAccounts
        {
            public PublicKey GlobalConfig { get; set; }

            public PublicKey Program { get; set; } = new PublicKey("F7Dr4bH5knKjzBj8fuRJT9QGtHLyQSWTnWxYetHDnWHA");
            public PublicKey ProgramData { get; set; }

            public PublicKey Signer { get; set; }
        }

        public static class SolStrikeProgram
        {
            public const string ID = "F7Dr4bH5knKjzBj8fuRJT9QGtHLyQSWTnWxYetHDnWHA";
            public static Solana.Unity.Rpc.Models.TransactionInstruction BuyChipWithSol(BuyChipWithSolAccounts accounts, ulong amount, PublicKey programId = null)
            {
                programId ??= new(ID);
                List<Solana.Unity.Rpc.Models.AccountMeta> keys = new()
                {Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.Buyer, true), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.GlobalConfig, false), Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.Treasury, false), Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.ChipMint, false), Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.BuyerChipAccount, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.TokenProgram, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.AssociatedTokenProgram, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.SystemProgram, false)};
                byte[] _data = new byte[1200];
                int offset = 0;
                _data.WriteU64(9180956995626968905UL, offset);
                offset += 8;
                _data.WriteU64(amount, offset);
                offset += 8;
                byte[] resultData = new byte[offset];
                Array.Copy(_data, resultData, offset);
                return new Solana.Unity.Rpc.Models.TransactionInstruction{Keys = keys, ProgramId = programId.KeyBytes, Data = resultData};
            }

            public static Solana.Unity.Rpc.Models.TransactionInstruction ClaimChips(ClaimChipsAccounts accounts, PublicKey programId = null)
            {
                programId ??= new(ID);
                List<Solana.Unity.Rpc.Models.AccountMeta> keys = new()
                {Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.Signer, true), Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.ClaimableRewardsAccount, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.ChipMint, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.Treasury, false), Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.TreasuryChipTokenAccount, false), Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.ClaimerChipAccount, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.TokenProgram, false)};
                byte[] _data = new byte[1200];
                int offset = 0;
                _data.WriteU64(1934180530880433553UL, offset);
                offset += 8;
                byte[] resultData = new byte[offset];
                Array.Copy(_data, resultData, offset);
                return new Solana.Unity.Rpc.Models.TransactionInstruction{Keys = keys, ProgramId = programId.KeyBytes, Data = resultData};
            }

            public static Solana.Unity.Rpc.Models.TransactionInstruction Initialize(InitializeAccounts accounts, ulong lamports_price, PublicKey programId = null)
            {
                programId ??= new(ID);
                List<Solana.Unity.Rpc.Models.AccountMeta> keys = new()
                {Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.GlobalConfig, false), Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.ChipMint, false), Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.Treasury, false), Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.TreasuryChipTokenAccount, false), Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.Signer, true), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.Program, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.ProgramData, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.TokenProgram, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.AssociatedTokenProgram, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.SystemProgram, false)};
                byte[] _data = new byte[1200];
                int offset = 0;
                _data.WriteU64(17121445590508351407UL, offset);
                offset += 8;
                _data.WriteU64(lamports_price, offset);
                offset += 8;
                byte[] resultData = new byte[offset];
                Array.Copy(_data, resultData, offset);
                return new Solana.Unity.Rpc.Models.TransactionInstruction{Keys = keys, ProgramId = programId.KeyBytes, Data = resultData};
            }

            public static Solana.Unity.Rpc.Models.TransactionInstruction ReserveChips(ReserveChipsAccounts accounts, ulong amount, PublicKey programId = null)
            {
                programId ??= new(ID);
                List<Solana.Unity.Rpc.Models.AccountMeta> keys = new()
                {Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.Signer, true), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.Treasury, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.ChipMint, false), Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.TreasuryChipTokenAccount, false), Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.UserChipAccount, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.TokenProgram, false)};
                byte[] _data = new byte[1200];
                int offset = 0;
                _data.WriteU64(2781749682280865997UL, offset);
                offset += 8;
                _data.WriteU64(amount, offset);
                offset += 8;
                byte[] resultData = new byte[offset];
                Array.Copy(_data, resultData, offset);
                return new Solana.Unity.Rpc.Models.TransactionInstruction{Keys = keys, ProgramId = programId.KeyBytes, Data = resultData};
            }

            public static Solana.Unity.Rpc.Models.TransactionInstruction SellChip(SellChipAccounts accounts, ulong amount, PublicKey programId = null)
            {
                programId ??= new(ID);
                List<Solana.Unity.Rpc.Models.AccountMeta> keys = new()
                {Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.Seller, true), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.GlobalConfig, false), Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.ChipMint, false), Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.Treasury, false), Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.SellerChipAccount, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.TokenProgram, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.AssociatedTokenProgram, false)};
                byte[] _data = new byte[1200];
                int offset = 0;
                _data.WriteU64(3332416062178962661UL, offset);
                offset += 8;
                _data.WriteU64(amount, offset);
                offset += 8;
                byte[] resultData = new byte[offset];
                Array.Copy(_data, resultData, offset);
                return new Solana.Unity.Rpc.Models.TransactionInstruction{Keys = keys, ProgramId = programId.KeyBytes, Data = resultData};
            }

            public static Solana.Unity.Rpc.Models.TransactionInstruction SetClaimableRewards(SetClaimableRewardsAccounts accounts, PublicKey programId = null)
            {
                programId ??= new(ID);
                List<Solana.Unity.Rpc.Models.AccountMeta> keys = new()
                {Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.Signer, true), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.Program, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.ProgramData, false), Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.FirstPlaceClaimableRewardsAccount, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.FirstPlaceAuthority, false), Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.SecondPlaceClaimableRewardsAccount, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.SecondPlaceAuthority, false), Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.ThirdPlaceClaimableRewardsAccount, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.ThirdPlaceAuthority, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.SystemProgram, false)};
                byte[] _data = new byte[1200];
                int offset = 0;
                _data.WriteU64(16119603077347428777UL, offset);
                offset += 8;
                byte[] resultData = new byte[offset];
                Array.Copy(_data, resultData, offset);
                return new Solana.Unity.Rpc.Models.TransactionInstruction{Keys = keys, ProgramId = programId.KeyBytes, Data = resultData};
            }

            public static Solana.Unity.Rpc.Models.TransactionInstruction UpdateSolChipPrice(UpdateSolChipPriceAccounts accounts, ulong new_price, PublicKey programId = null)
            {
                programId ??= new(ID);
                List<Solana.Unity.Rpc.Models.AccountMeta> keys = new()
                {Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.GlobalConfig, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.Program, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.ProgramData, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.Signer, true)};
                byte[] _data = new byte[1200];
                int offset = 0;
                _data.WriteU64(11697544153095196084UL, offset);
                offset += 8;
                _data.WriteU64(new_price, offset);
                offset += 8;
                byte[] resultData = new byte[offset];
                Array.Copy(_data, resultData, offset);
                return new Solana.Unity.Rpc.Models.TransactionInstruction{Keys = keys, ProgramId = programId.KeyBytes, Data = resultData};
            }
        }
    }
}