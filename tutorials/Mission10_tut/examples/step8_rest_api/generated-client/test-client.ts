import { 
    OperationsApi, 
    UnionFindManagementApi,
    Configuration 
} from './typescript';

const config = new Configuration({
    basePath: 'http://127.0.0.1:8080'
});

const mgmtApi = new UnionFindManagementApi(config);
const opsApi = new OperationsApi(config);

async function testClient() {
    try {
        console.log('🧪 Testing Union-Find REST API Client\n');
        
        // 1. Create instance
        console.log('📝 Creating Union-Find instance with 10 elements...');
        const createResp = await mgmtApi.createInstance({
            size: 10
        });
        console.log(`✅ Created instance: ${createResp.data.id}`);
        console.log(`   Size: ${createResp.data.size}\n`);
        
        const id = createResp.data.id;
        
        // 2. Get initial stats
        console.log('📊 Getting initial stats...');
        const initialStats = await opsApi.getStats(id);
        console.log(`   Elements: ${initialStats.data.total_elements}`);
        console.log(`   Components: ${initialStats.data.num_components}\n`);
        
        // 3. Union elements
        console.log('🔗 Unioning elements 0 and 5...');
        const unionResp = await opsApi.unionElements(id, {
            element1: 0,
            element2: 5
        });
        console.log(`   Merged: ${unionResp.data.merged}`);
        console.log(`   Root: ${unionResp.data.root}\n`);
        
        // 4. Check connected
        console.log('🔍 Checking if 0 and 5 are connected...');
        const connectedResp = await opsApi.checkConnected(id, 0, 5);
        console.log(`   Connected: ${connectedResp.data.connected}\n`);
        
        // 5. Find root
        console.log('🌳 Finding root of element 5...');
        const findResp = await opsApi.findElement(id, 5);
        console.log(`   Element: ${findResp.data.element}`);
        console.log(`   Root: ${findResp.data.root}\n`);
        
        // 6. Get updated stats
        console.log('📊 Getting updated stats...');
        const updatedStats = await opsApi.getStats(id);
        console.log(`   Elements: ${updatedStats.data.total_elements}`);
        console.log(`   Components: ${updatedStats.data.num_components}\n`);
        
        // 7. Delete instance
        console.log('🗑️  Deleting instance...');
        await mgmtApi.deleteInstance(id);
        console.log('✅ Instance deleted successfully\n');
        
        console.log('🎉 All tests passed!');
        
    } catch (error: any) {
        console.error('❌ Error:', error.response?.data || error.message);
        process.exit(1);
    }
}

testClient();
