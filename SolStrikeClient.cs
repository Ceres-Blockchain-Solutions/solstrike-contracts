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
using SolStrike.Types;

namespace SolStrike
{
    namespace Accounts
    {
        public partial class ChipTokenPriceState
        {
            public static ulong ACCOUNT_DISCRIMINATOR => 7097304789661935891UL;
            public static ReadOnlySpan<byte> ACCOUNT_DISCRIMINATOR_BYTES => new byte[]{19, 105, 122, 98, 26, 177, 126, 98};
            public static string ACCOUNT_DISCRIMINATOR_B58 => "4FKcfS6ERmK";
            public PublicKey TokenAddress { get; set; }

            public ulong TokenPrice { get; set; }

            public byte Bump { get; set; }

            public static ChipTokenPriceState Deserialize(ReadOnlySpan<byte> _data)
            {
                int offset = 0;
                ulong accountHashValue = _data.GetU64(offset);
                offset += 8;
                if (accountHashValue != ACCOUNT_DISCRIMINATOR)
                {
                    return null;
                }

                ChipTokenPriceState result = new ChipTokenPriceState();
                result.TokenAddress = _data.GetPubKey(offset);
                offset += 32;
                result.TokenPrice = _data.GetU64(offset);
                offset += 8;
                result.Bump = _data.GetU8(offset);
                offset += 1;
                return result;
            }
        }

        public partial class GlobalConfig
        {
            public static ulong ACCOUNT_DISCRIMINATOR => 15686315269655627925UL;
            public static ReadOnlySpan<byte> ACCOUNT_DISCRIMINATOR_BYTES => new byte[]{149, 8, 156, 202, 160, 252, 176, 217};
            public static string ACCOUNT_DISCRIMINATOR_B58 => "Rvp9zjtEEBA";
            public ulong SolChipPrice { get; set; }

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
                result.SolChipPrice = _data.GetU64(offset);
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

    namespace Types
    {
    }

    public partial class SolStrikeClient : TransactionalBaseClient<SolStrikeErrorKind>
    {
        public SolStrikeClient(IRpcClient rpcClient, IStreamingRpcClient streamingRpcClient, PublicKey programId = null) : base(rpcClient, streamingRpcClient, programId ?? new PublicKey(SolStrikeProgram.ID))
        {
        }

        public async Task<Solana.Unity.Programs.Models.ProgramAccountsResultWrapper<List<ChipTokenPriceState>>> GetChipTokenPriceStatesAsync(string programAddress = SolStrikeProgram.ID, Commitment commitment = Commitment.Confirmed)
        {
            var list = new List<Solana.Unity.Rpc.Models.MemCmp>{new Solana.Unity.Rpc.Models.MemCmp{Bytes = ChipTokenPriceState.ACCOUNT_DISCRIMINATOR_B58, Offset = 0}};
            var res = await RpcClient.GetProgramAccountsAsync(programAddress, commitment, memCmpList: list);
            if (!res.WasSuccessful || !(res.Result?.Count > 0))
                return new Solana.Unity.Programs.Models.ProgramAccountsResultWrapper<List<ChipTokenPriceState>>(res);
            List<ChipTokenPriceState> resultingAccounts = new List<ChipTokenPriceState>(res.Result.Count);
            resultingAccounts.AddRange(res.Result.Select(result => ChipTokenPriceState.Deserialize(Convert.FromBase64String(result.Account.Data[0]))));
            return new Solana.Unity.Programs.Models.ProgramAccountsResultWrapper<List<ChipTokenPriceState>>(res, resultingAccounts);
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

        public async Task<Solana.Unity.Programs.Models.AccountResultWrapper<ChipTokenPriceState>> GetChipTokenPriceStateAsync(string accountAddress, Commitment commitment = Commitment.Finalized)
        {
            var res = await RpcClient.GetAccountInfoAsync(accountAddress, commitment);
            if (!res.WasSuccessful)
                return new Solana.Unity.Programs.Models.AccountResultWrapper<ChipTokenPriceState>(res);
            var resultingAccount = ChipTokenPriceState.Deserialize(Convert.FromBase64String(res.Result.Value.Data[0]));
            return new Solana.Unity.Programs.Models.AccountResultWrapper<ChipTokenPriceState>(res, resultingAccount);
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

        public async Task<SubscriptionState> SubscribeChipTokenPriceStateAsync(string accountAddress, Action<SubscriptionState, Solana.Unity.Rpc.Messages.ResponseValue<Solana.Unity.Rpc.Models.AccountInfo>, ChipTokenPriceState> callback, Commitment commitment = Commitment.Finalized)
        {
            SubscriptionState res = await StreamingRpcClient.SubscribeAccountInfoAsync(accountAddress, (s, e) =>
            {
                ChipTokenPriceState parsingResult = null;
                if (e.Value?.Data?.Count > 0)
                    parsingResult = ChipTokenPriceState.Deserialize(Convert.FromBase64String(e.Value.Data[0]));
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
        public class AddTokenAccounts
        {
            public PublicKey ChipTokenPriceState { get; set; }

            public PublicKey Treasury { get; set; }

            public PublicKey TreasuryTokenAccount { get; set; }

            public PublicKey Signer { get; set; }

            public PublicKey PaymentTokenMint { get; set; }

            public PublicKey TokenProgram { get; set; }

            public PublicKey AssociatedTokenProgram { get; set; } = new PublicKey("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");
            public PublicKey SystemProgram { get; set; } = new PublicKey("11111111111111111111111111111111");
        }

        public class BuyChipAccounts
        {
            public PublicKey ChipTokenPriceState { get; set; }

            public PublicKey Treasury { get; set; }

            public PublicKey TreasuryTokenAccount { get; set; }

            public PublicKey Buyer { get; set; }

            public PublicKey ChipMint { get; set; }

            public PublicKey BuyerChipAccount { get; set; }

            public PublicKey BuyerTokenAccount { get; set; }

            public PublicKey PaymentTokenMint { get; set; }

            public PublicKey TokenProgram { get; set; }

            public PublicKey AssociatedTokenProgram { get; set; } = new PublicKey("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");
            public PublicKey SystemProgram { get; set; } = new PublicKey("11111111111111111111111111111111");
        }

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

        public class InitGlobalConfigAccounts
        {
            public PublicKey Signer { get; set; }

            public PublicKey GlobalConfig { get; set; }

            public PublicKey SystemProgram { get; set; } = new PublicKey("11111111111111111111111111111111");
        }

        public class InitializeAccounts
        {
            public PublicKey ChipMint { get; set; }

            public PublicKey Treasury { get; set; }

            public PublicKey Signer { get; set; }

            public PublicKey TokenProgram { get; set; }

            public PublicKey SystemProgram { get; set; } = new PublicKey("11111111111111111111111111111111");
        }

        public class SellChipAccounts
        {
        }

        public class UpdateChipTokenPriceAccounts
        {
            public PublicKey ChipTokenPriceState { get; set; }

            public PublicKey Signer { get; set; }

            public PublicKey TokenMint { get; set; }
        }

        public class UpdateSolChipPriceAccounts
        {
            public PublicKey GlobalConfig { get; set; }

            public PublicKey Signer { get; set; }
        }

        public static class SolStrikeProgram
        {
            public const string ID = "3FFYCYGMqkjjpxMvGXu5XiRnZQtGJMN9r73Hh1yiBVjH";
            public static Solana.Unity.Rpc.Models.TransactionInstruction AddToken(AddTokenAccounts accounts, ulong token_price, PublicKey programId = null)
            {
                programId ??= new(ID);
                List<Solana.Unity.Rpc.Models.AccountMeta> keys = new()
                {Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.ChipTokenPriceState, false), Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.Treasury, false), Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.TreasuryTokenAccount, false), Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.Signer, true), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.PaymentTokenMint, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.TokenProgram, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.AssociatedTokenProgram, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.SystemProgram, false)};
                byte[] _data = new byte[1200];
                int offset = 0;
                _data.WriteU64(3766188206372618221UL, offset);
                offset += 8;
                _data.WriteU64(token_price, offset);
                offset += 8;
                byte[] resultData = new byte[offset];
                Array.Copy(_data, resultData, offset);
                return new Solana.Unity.Rpc.Models.TransactionInstruction{Keys = keys, ProgramId = programId.KeyBytes, Data = resultData};
            }

            public static Solana.Unity.Rpc.Models.TransactionInstruction BuyChip(BuyChipAccounts accounts, ulong amount, PublicKey programId = null)
            {
                programId ??= new(ID);
                List<Solana.Unity.Rpc.Models.AccountMeta> keys = new()
                {Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.ChipTokenPriceState, false), Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.Treasury, false), Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.TreasuryTokenAccount, false), Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.Buyer, true), Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.ChipMint, false), Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.BuyerChipAccount, false), Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.BuyerTokenAccount, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.PaymentTokenMint, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.TokenProgram, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.AssociatedTokenProgram, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.SystemProgram, false)};
                byte[] _data = new byte[1200];
                int offset = 0;
                _data.WriteU64(5593750255586685921UL, offset);
                offset += 8;
                _data.WriteU64(amount, offset);
                offset += 8;
                byte[] resultData = new byte[offset];
                Array.Copy(_data, resultData, offset);
                return new Solana.Unity.Rpc.Models.TransactionInstruction{Keys = keys, ProgramId = programId.KeyBytes, Data = resultData};
            }

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

            public static Solana.Unity.Rpc.Models.TransactionInstruction InitGlobalConfig(InitGlobalConfigAccounts accounts, ulong chip_price, PublicKey programId = null)
            {
                programId ??= new(ID);
                List<Solana.Unity.Rpc.Models.AccountMeta> keys = new()
                {Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.Signer, true), Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.GlobalConfig, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.SystemProgram, false)};
                byte[] _data = new byte[1200];
                int offset = 0;
                _data.WriteU64(18408463851358423180UL, offset);
                offset += 8;
                _data.WriteU64(chip_price, offset);
                offset += 8;
                byte[] resultData = new byte[offset];
                Array.Copy(_data, resultData, offset);
                return new Solana.Unity.Rpc.Models.TransactionInstruction{Keys = keys, ProgramId = programId.KeyBytes, Data = resultData};
            }

            public static Solana.Unity.Rpc.Models.TransactionInstruction Initialize(InitializeAccounts accounts, PublicKey programId = null)
            {
                programId ??= new(ID);
                List<Solana.Unity.Rpc.Models.AccountMeta> keys = new()
                {Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.ChipMint, false), Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.Treasury, false), Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.Signer, true), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.TokenProgram, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.SystemProgram, false)};
                byte[] _data = new byte[1200];
                int offset = 0;
                _data.WriteU64(17121445590508351407UL, offset);
                offset += 8;
                byte[] resultData = new byte[offset];
                Array.Copy(_data, resultData, offset);
                return new Solana.Unity.Rpc.Models.TransactionInstruction{Keys = keys, ProgramId = programId.KeyBytes, Data = resultData};
            }

            public static Solana.Unity.Rpc.Models.TransactionInstruction SellChip(SellChipAccounts accounts, ulong amount, PublicKey receive_token, PublicKey programId = null)
            {
                programId ??= new(ID);
                List<Solana.Unity.Rpc.Models.AccountMeta> keys = new()
                {};
                byte[] _data = new byte[1200];
                int offset = 0;
                _data.WriteU64(3332416062178962661UL, offset);
                offset += 8;
                _data.WriteU64(amount, offset);
                offset += 8;
                _data.WritePubKey(receive_token, offset);
                offset += 32;
                byte[] resultData = new byte[offset];
                Array.Copy(_data, resultData, offset);
                return new Solana.Unity.Rpc.Models.TransactionInstruction{Keys = keys, ProgramId = programId.KeyBytes, Data = resultData};
            }

            public static Solana.Unity.Rpc.Models.TransactionInstruction UpdateChipTokenPrice(UpdateChipTokenPriceAccounts accounts, ulong new_token_price, PublicKey programId = null)
            {
                programId ??= new(ID);
                List<Solana.Unity.Rpc.Models.AccountMeta> keys = new()
                {Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.ChipTokenPriceState, false), Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.Signer, true), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.TokenMint, false)};
                byte[] _data = new byte[1200];
                int offset = 0;
                _data.WriteU64(14567175082992295059UL, offset);
                offset += 8;
                _data.WriteU64(new_token_price, offset);
                offset += 8;
                byte[] resultData = new byte[offset];
                Array.Copy(_data, resultData, offset);
                return new Solana.Unity.Rpc.Models.TransactionInstruction{Keys = keys, ProgramId = programId.KeyBytes, Data = resultData};
            }

            public static Solana.Unity.Rpc.Models.TransactionInstruction UpdateSolChipPrice(UpdateSolChipPriceAccounts accounts, ulong new_price, PublicKey programId = null)
            {
                programId ??= new(ID);
                List<Solana.Unity.Rpc.Models.AccountMeta> keys = new()
                {Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.GlobalConfig, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.Signer, true)};
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